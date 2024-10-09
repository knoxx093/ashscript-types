use std::default;

use hashbrown::HashMap;
use hexx::Hex;
use serde::Serialize;

use crate::{constants::{CHUNK_SIZE, HEX_LAYOUT}, structures::{factory::Factories, turret::Turrets}, unit::{Unit, Units}};

pub type Chunks = HashMap<Hex, Chunk>;

#[derive(Default, Serialize)]
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

#[derive(Default)]
pub struct Map {
    pub chunks: Chunks
}

impl Map {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn chunk_at(&self, hex: &Hex) -> Option<&Chunk> {
        let chunk_hex = hex.to_lower_res(CHUNK_SIZE);
        self.chunks.get(&chunk_hex)
    }

    pub fn unit_at(&self, hex: &Hex) -> Option<&Unit> {
        let chunk = self.chunk_at(hex)?;

        chunk.units.get(hex)
    }
}