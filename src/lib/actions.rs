use crate::lib::display::*;
use ncurses::*;
use std::io;

use super::{state::save_state, Todo::*};

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

pub fn save_and_exit(todo_list: &Vec<Todo>) -> bool {
  let save_result = save_state(&todo_list);
  return save_result;
}

pub fn delete(todo_list: &mut Vec<Todo>, index: usize) {
  if todo_list.len() > 1 {
    todo_list.remove(index);
  }
}
