use hashbrown::HashMap;
use hexx::Hex;
use serde::Serialize;
use uuid::Uuid;

use crate::{objects::{GameObjectKind, HasHealth, HasHex, HasId, HasStorage}, player::OwnerId, storage::Storage};

pub type Factories = HashMap<Hex, Factory>;

#[derive(Serialize, Default, Clone)]
pub struct Factory {
    pub id: Uuid,
    pub kind: GameObjectKind,
    pub owner_id: OwnerId,
    pub energy: u32,
    pub hex: Hex,
    pub progress: u32,
    pub health: u32,
    pub storage: Storage,
    #[serde(skip)]
    pub future_health: u32,
    #[serde(skip)]
    pub future_energy: u32,
}

impl HasHealth for Factory {
    fn health(&self) -> u32 {
        self.health
    }
}

impl HasId for Factory {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl HasHex for Factory {
    fn hex(&self) -> Hex {
        self.hex
    }
}

impl HasStorage for Factory {
    fn storage(&self) -> &Storage {
        &self.storage
    }
}

impl Factory {
    
}