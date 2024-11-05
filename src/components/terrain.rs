use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Terrain;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Wall;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Lava;
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Plain;