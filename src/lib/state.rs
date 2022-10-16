use crate::lib::Todo::*;

pub fn load_state() -> Vec<Todo> {
  return Vec::from([
    Todo::new("Read book".to_string(), false),
    Todo::new("Play with the kid".to_string(), false),
    Todo::new("Walk the wife".to_string(), false),
    Todo::new("1".to_string(), false),
    Todo::new("2".to_string(), false),
    Todo::new("3".to_string(), false),
    Todo::new("1".to_string(), false),
    Todo::new("2".to_string(), false),
    Todo::new("3".to_string(), false),
    Todo::new("1".to_string(), false),
    Todo::new("2".to_string(), false),
    Todo::new("EOF the state".to_string(), false),
  ]);
}
