use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TodoList(Vec<Todo>);

impl TodoList {
  pub fn new(todos: Vec<Todo>) -> TodoList {
    TodoList(todos)
  }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Todo {
  text: String,
  done: bool,
}

impl Todo {
  pub fn new(text: String, status: bool) -> Todo {
    Todo { text, done: status }
  }

  pub fn display(&self) -> &String {
    return &self.text;
  }

  pub fn is_done(&self) -> &bool {
    return &self.done;
  }

  pub fn update_status(&mut self, status: bool) {
    self.done = status;
  }
}
