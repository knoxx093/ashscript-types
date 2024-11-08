use std::default;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Terrain {
    pub kind: TerrainKind,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub enum TerrainKind {
    Wall,
    Lava,
    #[default]
    Plain,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Wall;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Lava;
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Plain;