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
    // TODO REFACTOR
    //---------------
    let max_y = get_screen_h();
    let reserved_space = 4;
    let border = max_y as usize - reserved_space;
    let mut list_limit = {
      if border < todo_list.len() {
        border
      } else {
        todo_list.len()
      }
    };

    //---------------------
    display_todo_list(&mut todo_list, todo_cur_index, list_limit);

    let key = getch();
    match key as u8 as char {
      'q' => quit = true,
      // ARROW KEYS
      // -----
      'A' => {
        if todo_cur_index != 0 {
          todo_cur_index -= 1;
          if todo_cur_index >= list_limit {
            todo_list.rotate_right(1);
          }
        }
      } // up
      'B' => {
        if todo_cur_index != todo_list.len() - 1 {
          todo_cur_index += 1;

          if todo_cur_index > list_limit {
            todo_list.rotate_left(1);
          }
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

fn get_screen_h() -> i32 {
  let mut max_x = 0; // with
  let mut max_y = 0; // height
  getmaxyx(stdscr(), &mut max_y, &mut max_x);
  return max_y;
}
