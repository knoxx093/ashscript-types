use hashbrown::HashMap;
use hexx::Hex;
use serde::{Deserialize, Serialize};

use crate::{
    structures::{
        assembler::Assemblers, distributor::Distributors, factory::Factories, turret::Turrets,
    },
    terrain::{CoalNode, MineralNode, Scrap, Terrain},
    unit::Units,
};

pub type Chunks = HashMap<Hex, Chunk>;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Chunk {
    pub chunk_hex: Hex,
    pub units: Units,
    pub turrets: Turrets,
    pub factories: Factories,
    pub assemblers: Assemblers,
    pub distributors: Distributors,
    pub terrain: HashMap<Hex, Terrain>,
    pub mineral_nodes: HashMap<Hex, MineralNode>,
    pub coal_nodes: HashMap<Hex, CoalNode>,
    pub scrap: HashMap<Hex, Scrap>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
