use enum_map::EnumMap;
use hashbrown::HashMap;
use hexx::Hex;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    constants::{
        general::{UNIT_PART_COSTS, UNIT_PART_WEIGHTS},
        unit::{AGE_PER_GEN_PART, UNIT_AGE_EXP, UNIT_BASE_AGE},
    },
    intents::{self, Intent, Intents},
    objects::{GameObjectKind, HasHealth, HasHex, HasId, HasStorage},
    player::PlayerId,
    resource::Resource,
    storage::Storage,
};

pub type Units = HashMap<Hex, Unit>;

#[derive(Default, Serialize, Clone)]
pub struct Unit {
    pub id: Uuid,
    pub kind: GameObjectKind,
    pub owner_id: PlayerId,
    pub name: String,
    pub health: u32,
    pub hex: Hex,
    pub energy: u32,
    pub age: u32,
    pub body: UnitBody,
    pub storage: Storage,
    #[serde(skip)]
    pub future_health: u32,
    #[serde(skip)]
    pub future_energy: u32,
    #[serde(skip)]
    pub future_hex: Hex,
}

impl HasHealth for Unit {
    fn health(&self) -> u32 {
        self.health
    }
}

impl HasId for Unit {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl HasHex for Unit {
    fn hex(&self) -> Hex {
        self.hex
    }
}

impl HasStorage for Unit {
    fn storage(&self) -> &Storage {
        &self.storage
    }
}

impl Unit {
    pub fn new(hex: Hex) -> Self {
        Self {
            health: 100,
            hex,
            ..Default::default()
        }
    }

    pub fn max_age(&self) -> u32 {
        ((self.body.0[UnitPart::Generate] * AGE_PER_GEN_PART) as f32).powf(UNIT_AGE_EXP) as u32
            + UNIT_BASE_AGE
    }

    pub fn weight(&self) -> u32 {
        let mut weight = 0;

        for (part, _) in UNIT_PART_WEIGHTS.iter() {
            weight += UNIT_PART_WEIGHTS[part]
        }

        weight
    }

    pub fn range(&self) -> u32 {
        self.body.0[UnitPart::Ranged]
    }

    pub fn damage(&self) -> u32 {
        self.body.0[UnitPart::Ranged]
    }

    pub fn attack_cost(&self) -> u32 {
        self.body.0[UnitPart::Ranged]
    }

    /* pub fn attack<T>(&self, target: T, intents: &mut Intents)
    where
        T: HasHealth + HasHex,
    {
        intents.push(Intent::UnitAttack(intents::UnitAttack {
            attacker_hex: self.hex,
            target_hex: target.hex(),
        }));
    }

    pub fn attack_checked<T>(&self, target: T, intents: &mut Intents)
    where
        T: HasHealth + HasHex,
    {
        // Checks see if the itnent is likely to be converted into an action

        self.attack(target, intents);
    } */
}

#[derive(Default, Clone, Copy, Serialize, Debug)]
pub struct UnitBody(pub EnumMap<UnitPart, u32>);

impl UnitBody {
    pub fn cost(&self) -> HashMap<Resource, u32> {
        let mut cost = HashMap::new();

        for (part_type, count) in self.0.iter() {
            let (resource, cost_per_part) = UNIT_PART_COSTS[part_type];
            cost.insert(resource, cost_per_part * count);
        }

        cost
    }

    pub fn weight(&self) -> u32 {
        let mut weight = 0;

        for (part_type, count) in self.0.iter() {
            weight += UNIT_PART_WEIGHTS[part_type] * count;
        }

        weight
    }
}

#[derive(enum_map::Enum, Serialize, Clone, Copy, Debug)]
pub enum UnitPart {
    Ranged,
    Harvest,
    Generate,
    Work,
    Battery,
    Shield,
    Fabricate,
    RangeImprovement,
    DamageImprovement,
    GenerateImprovement,
    BatteryImprovement,
    ShieldImprovement,
    FabricateImprovement,
    WeightImprovement,
}
