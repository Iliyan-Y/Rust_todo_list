use ncurses::*;

use super::Todo::Todo;

const REGULAR: i16 = 0;
const HIGHLIGHT: i16 = 1;

pub fn init_ncurses() {
  initscr();
  noecho();
  start_color();
  init_pair(REGULAR, COLOR_WHITE, COLOR_BLACK);
  init_pair(HIGHLIGHT, COLOR_BLACK, COLOR_GREEN);
  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
  clear();
}

pub fn clean_up_terminal() {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn display_todo_list(
  todo_list: &Vec<Todo>,
  todo_cur_index: usize,
  list_limit: usize,
  list_view_cycle: i32,
) {
  let list_to_display = render_list_to_display(todo_list, list_view_cycle);
  clear();
  for (index, todo) in list_to_display.iter().enumerate() {
    let pair = {
      if todo_cur_index == index || (index == list_limit && todo_cur_index > list_limit) {
        HIGHLIGHT
      } else {
        REGULAR
      }
    };

    attron(COLOR_PAIR(pair));
    mv(index as i32, 1);
    let output = {
      if *todo.is_done() {
        format!("[X]{}", todo.display())
      } else {
        format!("[ ]{}", todo.display())
      }
    };
    addstr(&output);
    attroff(COLOR_PAIR(pair));

    if index >= list_limit {
      break;
    }
  }
  render_info(todo_list.len() as i32);
  refresh();
}

fn render_list_to_display(todo_list: &Vec<Todo>, list_view_cycle: i32) -> Vec<Todo> {
  match list_view_cycle {
    1 => {
      return todo_list
        .iter()
        .cloned()
        .filter(|f| !*f.is_done())
        .collect()
    }
    -1 => return todo_list.iter().cloned().filter(|f| *f.is_done()).collect(),
    _ => return todo_list.clone(),
  }
}

fn render_info(list_length: i32) {
  mv(list_length + 1, 1);
  addstr("\n-------------------MENU----------------------");
  addstr("\n ENTER = new; r = remove;\n SPACE = done/undone; q = quit");
}
