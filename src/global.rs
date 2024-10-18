use hashbrown::HashMap;
use serde::Serialize;

use crate::player::{Player, PlayerId};


#[derive(Default, Serialize)]
pub struct Global {
    pub tick: u64,
    pub players: HashMap<PlayerId, Player>,
}

impl Global {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}