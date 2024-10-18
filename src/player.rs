use hashbrown::HashMap;
use serde::Serialize;
use uuid::Uuid;

pub type PlayerId = Uuid;

#[derive(Default, Serialize, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub public_memory: HashMap<String, String>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}