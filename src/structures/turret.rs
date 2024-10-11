use hashbrown::HashMap;
use hexx::Hex;
use serde::Serialize;

use crate::{id_type::{Attackable, HasStorage}, player::OwnerId};

pub type Turrets = HashMap<Hex, Turret>;

#[derive(Serialize, Default)]
pub struct Turret {
    pub owner_id: OwnerId,
    pub energy: u32,
    pub hex: Hex,
}

impl Attackable for Turret {}
impl HasStorage for Turret {}

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