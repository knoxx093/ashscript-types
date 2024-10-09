use hashbrown::HashMap;
use hexx::Hex;
use serde::Serialize;

pub type Factories = HashMap<Hex, Factory>;

#[derive(Serialize, Default)]
pub struct Factory {
    pub owner_id: u32,
    pub energy: u32,
    pub hex: Hex,
    pub progress: u32,
}

impl Factory {
    
}