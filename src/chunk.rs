
use hashbrown::HashMap;
use hexx::Hex;
use serde::Serialize;

use crate::{structures::{factory::Factories, turret::Turrets}, unit::Units};

pub type Chunks = HashMap<Hex, Chunk>;

#[derive(Default, Serialize, Clone)]
pub struct Chunk {
    pub chunk_hex: Hex,
    pub units: Units,
    pub turrets: Turrets,
    pub factories: Factories,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}