use hashbrown::HashMap;
use serde::Serialize;

pub type PlayerId = String;

#[derive(Default, Serialize, Debug)]
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