use serde::{Deserialize, Serialize};

pub mod api;

#[derive(Serialize, Deserialize, Debug)]
pub struct Actor {
    id: u32,
}

impl Actor {
    pub fn new(id: u32) -> Actor {
        Actor { id }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
