use crate::lib::Todo::*;
use std::fs;

pub fn load_state() -> Vec<Todo> {
  // TODO:  read from json

  // work with files https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x

  //bincode https://crates.io/crates/bincode

  // decode let decoded: World = bincode::deserialize(&encoded[..]).unwrap();
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
  let encoded: Vec<u8> = bincode::serialize(&todo_list[0]).unwrap();
  // let data = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
  fs::write("test", encoded).expect("Unable to write file");
  return true;
}
