
use hashbrown::HashMap;
use hexx::Hex;
use serde::Serialize;

use crate::{structures::{factory::Factories, turret::Turrets}, terrain::{CoalNode, MineralNode, Scrap, Terrain}, unit::Units};

pub type Chunks = HashMap<Hex, Chunk>;

#[derive(Default, Serialize, Clone)]
pub struct Chunk {
    pub chunk_hex: Hex,
    pub units: Units,
    pub turrets: Turrets,
    pub factories: Factories,
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