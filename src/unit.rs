use enum_map::{enum_map, EnumMap};
use hashbrown::HashMap;
use hexx::Hex;
use serde::Serialize;

use crate::chunk::Chunk;

#[derive(Default, Serialize)]
pub struct Unit {
    pub id: u32,
    pub owner_id: u32,
    pub health: u32,
    pub hex: Hex,
    pub energy: u32,
    pub age: u32,
    pub body: UnitBody,
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

pub type UnitBody = EnumMap<UnitPart, u32>;

#[derive(enum_map::Enum, Serialize)]
pub enum UnitPart {
    Ranged,
    Harvest,
    Generate,
    Work,
    Battery,
}
