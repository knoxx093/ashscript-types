use serde::Serialize;

#[derive(Default, Serialize, Clone, Debug)]
/// Tiles with consistent properties
pub enum Terrain {
    #[default]
    Plain,
    Wall,
    Lava,
}

#[derive(Default, Serialize, Clone, Debug)]
pub struct MineralNode {
    pub amount: u32,
}

#[derive(Default, Serialize, Clone, Debug)]
pub struct CoalNode {
    pub amount: u32,
}

#[derive(Default, Serialize, Clone, Debug)]
pub struct Scrap {
    pub amount: u32,
}