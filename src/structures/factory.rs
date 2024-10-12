use hashbrown::HashMap;
use hexx::Hex;
use serde::Serialize;

use crate::{id::UUID, objects::{HasHealth, HasId}, player::OwnerId};

pub type Factories = HashMap<Hex, Factory>;

#[derive(Serialize, Default)]
pub struct Factory {
    pub id: UUID,
    pub owner_id: OwnerId,
    pub energy: u32,
    pub hex: Hex,
    pub progress: u32,
    pub health: u32,
}

impl HasHealth for Factory {
    fn health(&self) -> u32 {
        self.health
    }
}

impl HasId for Factory {
    fn id(&self) -> UUID {
        self.id
    }
}

impl Factory {
    
}