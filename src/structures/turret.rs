use hashbrown::HashMap;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{objects::{GameObjectKind, HasHealth, HasHex, HasId, HasStorage}, player::PlayerId, storage::Storage};

pub type Turrets = HashMap<Hex, Turret>;

#[derive(Serialize, Default, Deserialize, Clone, Debug)]
pub struct Turret {
    pub id: Uuid,
    pub kind: GameObjectKind,
    pub owner_id: PlayerId,
    pub energy: u32,
    pub hex: Hex,
    pub health: u32,
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