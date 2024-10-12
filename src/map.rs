use hexx::Hex;
use serde::Serialize;

use crate::{chunk::{Chunk, Chunks}, constants::map::CHUNK_SIZE, unit::Unit};

#[derive(Default, Serialize, Clone)]
pub struct Map {
    pub chunks: Chunks,
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
