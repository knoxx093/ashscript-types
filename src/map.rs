use std::any::TypeId;

use hashbrown::HashMap;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    chunk::{Chunk, Chunks},
    constants::map::CHUNK_SIZE,
    entity::Component,
    objects::GameObjectKind,
    structures::{factory::Factory, turret::Turret},
    unit::Unit,
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
