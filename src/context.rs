use tokio::sync::{Mutex, MutexGuard};

use crate::actor::Actor;
use crate::data::ContextData;
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct AppContext(Arc<Mutex<Context>>);

impl AppContext {
    pub fn init() -> AppContext {
        AppContext(Arc::new(Mutex::new(Context::new())))
    }

    pub async fn trigger_action(&self, id: u32) -> () {
        let mut lock = self.lock().await;

        if let Some(a) = lock.actors.get_mut(&id) {
            a.act(self.clone());
        }
    }

    pub async fn lock(&self) -> MutexGuard<'_, Context> {
        self.0.lock().await
    }
}

pub struct Context {
    actors: HashMap<u32, Actor>,
    shared_data: ContextData
}

impl Context {
    pub fn new() -> Context {
        Context {
            actors: HashMap::new(),
            shared_data: ContextData::new()
        }
    }

    pub fn add_actor(&mut self) {
        let count = self.actors.len() as u32;
        let actor = Actor::new(count + 1);
        self.actors.insert(actor.get_id(), actor);
    }

    pub fn get_actor(&self, id: u32) -> Option<&Actor> {
        self.actors.get(&id)
    }

    pub fn get_actors(&self) -> Vec<&Actor> {
        self.actors.values().clone().collect()
    }

    pub fn get_data(&self) -> &ContextData {
        &self.shared_data
    }

    pub fn get_data_mut(&mut self) -> &mut ContextData {
        &mut self.shared_data
    }
}
