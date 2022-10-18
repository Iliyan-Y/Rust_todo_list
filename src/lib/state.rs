use crate::lib::Todo::*;
use std::fs;
use std::path::Path;

pub fn load_state() -> Vec<Todo> {
  let file_exist = Path::new("./TodoListState").exists();
  if !file_exist {
    return vec![Todo::new(
      "Press enter to add your first todo".to_string(),
      false,
    )];
  }

  let data = fs::read("./TodoListState").expect("Unable to read file");
  let decoded: Vec<Todo> = bincode::deserialize(&data[..]).unwrap();
  return decoded;
}

pub fn save_state(todo_list: &Vec<Todo>) -> bool {
  let encoded: Vec<u8> = bincode::serialize(&todo_list).unwrap();
  fs::write("TodoListState", encoded).expect("Unable to write file");

  return true;
}
