use hashbrown::HashMap;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{objects::{GameObjectKind}, player::PlayerId};

pub type Turrets = HashMap<Hex, Turret>;

#[derive(Serialize, Default, Deserialize, Clone, Debug)]
pub struct Turret;

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