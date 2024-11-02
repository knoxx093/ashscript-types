use hashbrown::HashMap;
use hexx::Hex;
use ::serde::{Deserialize, Serialize};
use uuid::{serde, Uuid};

use crate::{objects::{GameObjectKind, HasHealth, HasHex, HasId, HasStorage}, player::PlayerId, storage::Storage};

pub type Assemblers = HashMap<Hex, Assembler>;

#[derive(Serialize, Default, Deserialize, Clone, Debug)]
pub struct Assembler {
    pub id: Uuid,
    pub kind: GameObjectKind,
    pub owner_id: PlayerId,
    pub hex: Hex,
    pub health: u32,
    pub storage: Storage,
    #[serde(skip)]
    pub future_health: u32,
}

impl HasHealth for Assembler {
    fn health(&self) -> u32 {
        self.health
    }
}

impl HasId for Assembler {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl HasHex for Assembler {
    fn hex(&self) -> Hex {
        self.hex
    }
}

impl HasStorage for Assembler {
    fn storage(&self) -> &Storage {
        &self.storage
    }
}