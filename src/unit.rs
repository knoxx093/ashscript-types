use enum_map::{enum_map, EnumMap};
use hashbrown::HashMap;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    constants::{
        general::{UNIT_PART_COSTS, UNIT_PART_WEIGHTS},
        unit::{AGE_PER_GEN_PART, BATTERY_ENERGY_CAPACITY, GENERATOR_ENERGY_CAPACITY, GENERATOR_ENERGY_INCOME, UNIT_AGE_EXP, UNIT_BASE_AGE, UNIT_HEALTH_PER_PART, UNIT_SHIELD_HEALTH},
    },
    intents::{self, Intent, Intents},
    objects::{GameObjectKind, HasHealth, HasHex, HasId, HasStorage},
    player::{Player, PlayerId},
    resource::Resource,
    storage::Storage,
};

pub type Units = HashMap<Hex, Unit>;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
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
    pub fn new(hex: Hex, name: String, body: UnitBody, owner_id: PlayerId) -> Self {
        Self {
            health: body.health_with_shields(),
            body,
            name,
            hex,
            owner_id,
            ..Default::default()
        }
    }

    pub fn max_age(&self) -> u32 {
        ((self.body.parts[UnitPart::Generate] * AGE_PER_GEN_PART) as f32).powf(UNIT_AGE_EXP) as u32
            + UNIT_BASE_AGE
    }

    pub fn range(&self) -> u32 {
        self.body.parts[UnitPart::Ranged]
    }

    pub fn damage(&self) -> u32 {
        self.body.parts[UnitPart::Ranged]
    }

    pub fn attack_cost(&self) -> u32 {
        self.body.parts[UnitPart::Ranged]
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

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct UnitBody {
    pub parts: EnumMap<UnitPart, u32>,
    pub shield_health: u32,
}

impl UnitBody {
    pub fn from_vec(parts: Vec<(UnitPart, u32)>) -> Self {
        let mut parts_map = enum_map! {
            _ => 0,
        };

        for (part, count) in parts {
            parts_map[part] += count;
        }

        Self {
            parts: parts_map,
            shield_health: parts_map[UnitPart::Shield] * UNIT_SHIELD_HEALTH,
        }
    }

    pub fn cost(&self) -> HashMap<Resource, u32> {
        let mut cost = HashMap::new();

        for (part_type, count) in self.parts.iter() {
            let (resource, cost_per_part) = UNIT_PART_COSTS[part_type];

            *cost.entry(resource).or_insert(0) += cost_per_part * count;
        }

        cost
    }

    pub fn weight(&self) -> u32 {
        let mut weight = 0;

        for (part, count) in self.parts.iter() {
            weight += UNIT_PART_WEIGHTS[part] * count;
        }

        weight
    }

    pub fn health_with_shields(&self) -> u32 {
        self.health_without_shields() + self.shield_health
    }

    pub fn health_without_shields(&self) -> u32 {
        let mut health = 0;

        for (part, count) in self.parts.iter() {
            health += count * UNIT_HEALTH_PER_PART
        }

        health
    }

    pub fn max_shield_health(&self) -> u32 {
        self.parts[UnitPart::Shield] * UNIT_SHIELD_HEALTH
    }

    pub fn energy_income(&self) -> u32 {
        self.parts[UnitPart::Generate] * GENERATOR_ENERGY_INCOME
    }

    pub fn energy_capacity(&self) -> u32 {
        self.parts[UnitPart::Generate] * GENERATOR_ENERGY_CAPACITY
            + self.parts[UnitPart::Battery] * BATTERY_ENERGY_CAPACITY
    }
}

#[derive(enum_map::Enum, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum UnitPart {
    Ranged,
    Harvest,
    Generate,
    Work,
    Battery,
    Shield,
    Fabricate,
    Convert,
    RangeImprovement,
    DamageImprovement,
    GenerateImprovement,
    BatteryImprovement,
    ShieldImprovement,
    FabricateImprovement,
    WeightImprovement,
}
