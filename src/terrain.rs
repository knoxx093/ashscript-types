use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
/// Tiles with consistent properties
pub enum Terrain {
    #[default]
    Plain,
    Wall,
    Lava,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct MineralNode {
    pub amount: u32,
}

#[derive(Default, Serialize, Deserialize,  Clone, Debug)]
pub struct CoalNode {
    pub amount: u32,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Scrap {
    pub amount: u32,
}