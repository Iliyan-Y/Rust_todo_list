use lib::actions::*;
use lib::display::*;
use lib::state::*;
use ncurses::*;

mod lib;

fn main() {
  init_ncurses();
  let mut quit = false;
  let mut todo_cur_index: usize = 0;
  let mut todo_list = load_state();
  while !quit {
    display_todo_list(&mut todo_list, todo_cur_index);
    let key = getch();
    match key as u8 as char {
      'q' => quit = true,
      // ARROW KEYS
      // -----
      'A' => {
        if todo_cur_index != 0 {
          todo_cur_index -= 1;
        }
      } // up
      'B' => {
        if todo_cur_index != todo_list.len() - 1 {
          todo_cur_index += 1
        }
      } // down
      'D' => quit = true, // left
      'C' => quit = true, // right
      // -----
      ' ' => quit = true,                      // SPACE
      '\n' => create_new_task(&mut todo_list), // ENTER
      _ => {
        // let key_as_char = key as u8;
        // addstr(&key_as_char.to_string());
      }
    }
  }

  endwin();
}
