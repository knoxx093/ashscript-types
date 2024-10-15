use hashbrown::HashMap;
use hexx::Hex;
use serde::Serialize;
use uuid::Uuid;

use crate::{objects::{GameObjectKind, HasHealth, HasHex, HasId, HasStorage}, player::OwnerId, storage::Storage};

pub type Turrets = HashMap<Hex, Turret>;

#[derive(Serialize, Default, Clone)]
pub struct Turret {
    pub id: Uuid,
    pub kind: GameObjectKind,
    pub owner_id: OwnerId,
    pub energy: u32,
    pub hex: Hex,
    pub health: u32,
    pub storage: Storage,
    #[serde(skip)]
    pub future_health: u32,
    #[serde(skip)]
    pub future_energy: u32,
}

impl HasHealth for Turret {
    fn health(&self) -> u32 {
        self.health
    }
}

impl HasId for Turret {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl HasHex for Turret {
    fn hex(&self) -> Hex {
        self.hex
    }
}

impl HasStorage for Turret {
    fn storage(&self) -> &Storage {
        &self.storage
    }
}

impl Turret {
    pub fn attack_cost(&self) -> u32 {
        self.range() + self.damage()
    }
    
    pub fn range(&self) -> u32 {
        1
    }
    
    pub fn damage(&self) -> u32 {
        1
    }
}