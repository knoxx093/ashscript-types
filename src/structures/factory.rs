use hashbrown::HashMap;
use hexx::Hex;
use serde::Serialize;

use crate::{id_type::{Attackable, HasStorage}, player::OwnerId};

pub type Factories = HashMap<Hex, Factory>;

#[derive(Serialize, Default)]
pub struct Factory {
    pub owner_id: OwnerId,
    pub energy: u32,
    pub hex: Hex,
    pub progress: u32,
}

impl Attackable for Factory {}
impl HasStorage for Factory {}

impl Factory {
    
}