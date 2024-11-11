use std::any::TypeId;

use hashbrown::HashMap;
use hecs::Entity;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    chunk::{Chunk, Chunks},
    constants::map::CHUNK_SIZE,
    entity::Component,
    objects::GameObjectKind,
};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Map {
    pub chunks: Chunks,
    pub data: MapData,
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

    pub fn chunk_at_mut(&mut self, hex: &Hex) -> Option<&mut Chunk> {
        let chunk_hex = hex.to_lower_res(CHUNK_SIZE);
        self.chunks.get_mut(&chunk_hex)
    }

    pub fn entity_at(&self, hex: &Hex, kind: GameObjectKind) -> Option<&Entity> {
        let chunk = self.chunk_at(hex)?;
        chunk.entities[kind].get(hex)
    }

    pub fn remove_entity(&mut self, hex: &Hex, kind: GameObjectKind) -> Option<Entity> {
        let chunk = self.chunk_at_mut(hex)?;
        chunk.entities[kind].remove(hex)
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct MapData {
    pub radius: u32,
}

impl MapData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
