use ncurses::*;

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

pub fn display_todo_list(todo_list: &mut Vec<String>, todo_cur_index: usize, list_limit: usize) {
  clear();

  for (index, todo) in todo_list.iter().enumerate() {
    let mut pair = {
      if todo_cur_index == index {
        HIGHLIGHT
      } else {
        REGULAR
      }
    };

    attron(COLOR_PAIR(pair));
    mv(index as i32, 1);
    addstr(todo);
    attroff(COLOR_PAIR(pair));

    let x = format!(
      "\n index {}, current_index {}, list_limit {}",
      index, todo_cur_index, list_limit,
    );
    addstr(&x);

    //TODO: refactor to pace the list
    //make the list "scrollable"
    if index >= list_limit {
      // mv(index as i32, 1);
      // addstr("....");
      break;
    }
  }
  render_info(todo_list.len() as i32);
  refresh();
}

fn render_info(list_length: i32) {
  mv(list_length + 1, 1);
  addstr("\n-------------------MENU----------------------");
  addstr("\n Press ENTER to add new task\n Press q to quit");
}
