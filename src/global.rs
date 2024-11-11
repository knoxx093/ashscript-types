use std::time::Duration;

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::player::{Player, PlayerId};


#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Global {
    pub tick: u64,
    pub last_tick_duration: Duration,
    pub players: HashMap<PlayerId, Player>,
}

impl Global {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}