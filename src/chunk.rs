use std::hash::Hash;

use enum_map::EnumMap;
use hashbrown::HashMap;
use hecs::Entity;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::objects::GameObjectKind;

pub type Chunks = HashMap<Hex, Chunk>;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Chunk {
    pub hex: Hex,
    pub entities: EnumMap<GameObjectKind, HashMap<Hex, Entity>>,
}

impl Chunk {
    pub fn new(hex: Hex) -> Self {
        Self {
            hex,
            ..Default::default()
        }
    }
}
