use enum_map::{enum_map, EnumMap};
use hashbrown::HashMap;
use hexx::Hex;

use crate::chunk::Chunk;

#[derive(Default)]
pub struct Unit {
    pub id: u32,
    pub health: u32,
    pub hex: Hex,
    pub energy: u32,
    pub age: u32,
}

impl Unit {
    pub fn new(hex: Hex) -> Self {
        Self {
            health: 100,
            hex,
            ..Default::default()
        }
    }
}

pub type UnitsByChunk = HashMap<u32, Chunk>;

#[derive(enum_map::Enum)]
pub enum UnitPart {
    Ranged,
    Harvest,
    Generate,
    Work,
    Battery,
}
