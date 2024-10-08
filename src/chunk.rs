use hashbrown::HashMap;

use crate::unit::Unit;

pub type Chunks = HashMap<ChunkId, Chunk>;

pub type ChunkId = u32;

#[derive(Default)]
pub struct Chunk {
    pub id: ChunkId,
    pub units: HashMap<u32, Unit>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}