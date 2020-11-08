use serde::{Deserialize, Serialize};

pub mod api;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextData {
  data : Vec<u8>
}

impl ContextData {
  pub fn new() -> ContextData {
    ContextData {
      data: Vec::new()
    }
  }

  pub fn add(&mut self, v : u8) {
    self.data.push(v);
  }

  pub fn get(&self, index : usize) -> Option<&u8> {
    self.data.get(index)
  }
}
