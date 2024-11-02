use hexx::Hex;
use ::serde::{Deserialize, Serialize};
use uuid::{serde, Uuid};

use crate::{objects::{GameObjectKind, HasHealth, HasHex, HasId, HasStorage}, player::PlayerId, storage::Storage};

#[derive(Serialize, Default, Deserialize, Clone, Debug)]
pub struct Distributor {
    pub id: Uuid,
    pub kind: GameObjectKind,
    pub owner_id: PlayerId,
    pub hex: Hex,
    pub health: u32,
    pub storage: Storage,
    #[serde(skip)]
    pub future_health: u32,
}

impl HasHealth for Distributor {
    fn health(&self) -> u32 {
        self.health
    }
}

impl HasId for Distributor {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl HasHex for Distributor {
    fn hex(&self) -> Hex {
        self.hex
    }
}

impl HasStorage for Distributor {
    fn storage(&self) -> &Storage {
        &self.storage
    }
}