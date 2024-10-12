use hashbrown::HashMap;
use hexx::Hex;
use serde::Serialize;
use uuid::Uuid;

use crate::{objects::{HasHealth, HasId}, player::OwnerId};

pub type Turrets = HashMap<Hex, Turret>;

#[derive(Serialize, Default)]
pub struct Turret {
    pub id: Uuid,
    pub owner_id: OwnerId,
    pub energy: u32,
    pub hex: Hex,
    pub health: u32,
}

impl HasHealth for Turret {
    fn health(&self) -> u32 {
        self.health
    }
}

impl HasId for Turret {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Turret {
    pub fn attack_cost(&self) -> u32 {
        self.range() + self.damage()
    }
    
    pub fn range(&self) -> u32 {
        1
    }
    
    pub fn damage(&self) -> u32 {
        1
    }
}