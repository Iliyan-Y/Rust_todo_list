use crate::lib::display::*;
use ncurses::*;
use std::io;

use super::Todo::*;

pub fn create_new_task(todo_list: &mut Vec<Todo>) {
  endwin();

  let mut new_task = String::new();
  clean_up_terminal();
  println!("Enter task name: ");
  io::stdin().read_line(&mut new_task).expect("error ");

  if !(new_task.trim().is_empty()) {
    todo_list.push(Todo::new(new_task, false));
  }

  init_ncurses();
}

pub fn change_task_state(todo_list: &mut Vec<Todo>, selected_index: usize) {
  let new_status = !todo_list[selected_index].is_done().clone();
  todo_list[selected_index].update_status(new_status);
}
