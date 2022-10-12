use ncurses::*;

const REGULAR: i16 = 0;
const HIGHLIGHT: i16 = 1;

pub fn display_todo_list(todo_list: &mut Vec<String>, todo_cur_index: usize) {
  for (index, todo) in todo_list.iter().enumerate() {
    let pair = {
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
  }

  mv(todo_list.len() as i32 + 1, 1);
  addstr("-------------------MENU----------------------");
  addstr("\n Press ENTER to add new task\n Press q to quit");
  refresh();
}
