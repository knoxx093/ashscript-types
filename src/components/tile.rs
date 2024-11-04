use hexx::Hex;
use serde::{Deserialize, Serialize};

use crate::entity::Component;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Tile {
    pub hex: Hex,
    pub previous_hex: Option<Hex>,
}