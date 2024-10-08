use hashbrown::HashMap;
use hexx::Hex;

use crate::chunk::Chunk;

#[derive(Default)]
pub struct Unit {
    pub id: u32,
    pub health: u32,
    pub hex: Hex,
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