use enum_map::{enum_map, EnumMap};
use hashbrown::HashMap;
use hexx::Hex;
use serde::Serialize;

use crate::{chunk::Chunk, constants::{general::UNIT_PART_WEIGHTS, unit::{AGE_PER_GEN_PART, UNIT_AGE_EXP, UNIT_BASE_AGE}}, id_type::{Attackable, HasStorage}, player::OwnerId, IdType};

pub type Units = HashMap<Hex, Unit>;

#[derive(Default, Serialize)]
pub struct Unit {
    pub id: IdType,
    pub owner_id: OwnerId,
    pub health: u32,
    pub hex: Hex,
    pub energy: u32,
    pub age: u32,
    pub body: UnitBody,
}

impl Attackable for Unit {}
impl HasStorage for Unit {}

impl Unit {
    pub fn new(hex: Hex) -> Self {
        Self {
            health: 100,
            hex,
            ..Default::default()
        }
    }

    pub fn max_age(&self) -> u32 {
        // TODO:
        // Fix this, this formula does NOT do what you intended.
        // y\ =\ \left(\left(x\ \cdot\ 10\right)^{0.4}\right)\ +\ 300
        // Copy and paste the line above into desmos and you will see what I mean.
        ((self.body[UnitPart::Generate] * AGE_PER_GEN_PART) as f32).powf(UNIT_AGE_EXP) as u32 + UNIT_BASE_AGE
    }

    // There were 3 functions here that returned things the user can calculate.
    // We provide a basic API, nothing else prefferably.

    pub fn weight(&self) -> u32 {
        let mut weight = 0;
    
        for (part, _) in UNIT_PART_WEIGHTS.iter() {
            weight += UNIT_PART_WEIGHTS[part]
        }
        
        weight
    }

    pub fn range(&self) -> u32 {
        self.body[UnitPart::Ranged]
    }
    
    pub fn damage(&self) -> u32 {
        self.body[UnitPart::Ranged]
    }
    
    pub fn attack_cost(&self) -> u32 {
        self.body[UnitPart::Ranged]
    }
}

pub type UnitBody = EnumMap<UnitPart, u32>;

#[derive(enum_map::Enum, Serialize)]
pub enum UnitPart {
    Ranged,
    Harvest,
    Generate,
    Work,
    Battery,
}
