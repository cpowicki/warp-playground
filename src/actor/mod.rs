use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

use crate::AppContext;

pub mod api;

#[derive(Serialize, Deserialize, Debug)]
pub enum ActorStatus {
    Sleeping,
    Acting,
    Broken,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actor {
    id: u32,

    status: ActorStatus,

    #[serde(skip)]
    handle: Option<JoinHandle<()>>,
}

impl Actor {
    pub fn new(id: u32) -> Actor {
        Actor {
            id,
            handle: None,
            status: ActorStatus::Sleeping,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn act(&mut self, context: AppContext) {
        let handle = tokio::spawn(async move {
            loop {
                let lock = context.lock().await;
                
                match lock.get_data().get(0) {
                    Some(val) if *val > (10 as u8) => {
                        // TODO update status 
                        println!("Terminating Action");
                        break;
                    }
                    _ => continue,
                }
            }
        });

        self.handle = Some(handle);
        self.status = ActorStatus::Acting;
    }
}
