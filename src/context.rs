use crate::actor::Actor;
use std::collections::HashMap;

pub struct Context {
    actors: HashMap<u32, Actor>,
}

impl Context {
    pub fn new() -> Context {
        let actors: HashMap<u32, Actor> = HashMap::new();
        Context { actors }
    }

    pub fn add_actor(&mut self) {
        let actor = Actor::new(1);
        self.actors.insert(actor.get_id(), actor);
    }

    pub fn get_actor(&self, id: u32) -> Option<&Actor> {
        self.actors.get(&id)
    }

    pub fn get_actors(&self) -> Vec<&Actor> {
        self.actors.values().clone().collect()
    }
}
