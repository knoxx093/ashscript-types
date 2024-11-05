use std::hash::Hash;

use hashbrown::HashMap;
use hecs::Entity;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    objects::GameObjectKind, structures::{
        assembler::Assemblers, distributor::Distributors, factory::Factories, turret::Turrets,
    }, unit::Units
};

pub type Chunks = HashMap<Hex, Chunk>;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Chunk {
    pub hex: Hex,
    pub entities: HashMap<GameObjectKind, HashMap<Hex, Entity>>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
