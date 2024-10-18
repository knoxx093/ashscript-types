use hashbrown::HashMap;
use hexx::Hex;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    constants::general::{IntentReturnCode, UNIT_PART_COSTS},
    intents::{self, Intent, Intents},
    objects::{GameObjectKind, HasHealth, HasHex, HasId, HasStorage},
    player::PlayerId,
    resource::Resource,
    storage::Storage,
    unit::{UnitBody, UnitPart},
};

pub type Factories = HashMap<Hex, Factory>;

#[derive(Serialize, Default, Clone)]
pub struct Factory {
    pub id: Uuid,
    pub kind: GameObjectKind,
    pub owner_id: PlayerId,
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
    pub fn spawn_unit(
        &self,
        name: String,
        parts: Vec<(UnitPart, u32)>,
        outs: Option<Vec<Hex>>,
        intents: &mut Intents,
    ) {
        intents.push(Intent::FactorySpawnUnit(intents::FactorySpawnUnit {
            factory_hex: self.hex,
            name,
            body: UnitBody::from_vec(parts),
            out: outs,
        }));
    }

    pub fn can_spawn_body(&self, parts: &[(UnitPart, u32)]) -> bool {
        let mut cost: HashMap<Resource, u32> = HashMap::new();
        for (part, part_amount) in parts.iter() {
            let (resource, resource_amount) = UNIT_PART_COSTS[*part];
            cost.insert(resource, resource_amount * part_amount);
        }

        self.storage.has_sufficient(&cost)
    }

    pub fn spawn_unit_checked(
        &self,
        name: String,
        parts: Vec<(UnitPart, u32)>,
        outs: Option<Vec<Hex>>,
        intents: &mut Intents,
    ) -> Result<(), IntentReturnCode> {
        if !self.can_spawn_body(&parts) {
            return Err(IntentReturnCode::InsufficientResources);
        }

        // We passed all the checks, create the intent

        self.spawn_unit(name, parts, outs, intents);

        Ok(())
    }
}
