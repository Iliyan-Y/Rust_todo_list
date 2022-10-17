use crate::lib::Todo::*;

pub fn load_state() -> Vec<Todo> {
  // TODO:  read from json

  return Vec::from([
    Todo::new("Read book".to_string(), false),
    Todo::new("Play with the kid".to_string(), true),
    Todo::new("Walk the wife".to_string(), false),
    Todo::new("1".to_string(), false),
    Todo::new("2".to_string(), true),
    Todo::new("3".to_string(), false),
    Todo::new("1".to_string(), false),
    Todo::new("2".to_string(), false),
    Todo::new("3".to_string(), false),
    Todo::new("1".to_string(), false),
    Todo::new("2".to_string(), true),
    Todo::new("EOF the state".to_string(), false),
  ]);
}

pub fn save_state(todo_list: &Vec<Todo>) -> bool {
  //TODO:  save the state before exist to json file
  // TODO: if save successful return true else false
  true
}
