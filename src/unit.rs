use enum_map::EnumMap;
use hashbrown::HashMap;
use hexx::Hex;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    constants::{
        general::UNIT_PART_WEIGHTS,
        unit::{AGE_PER_GEN_PART, UNIT_AGE_EXP, UNIT_BASE_AGE},
    },
    intents::{self, Intent, Intents},
    objects::{HasHealth, HasId},
    player::OwnerId,
};

pub type Units = HashMap<Hex, Unit>;

#[derive(Default, Serialize)]
pub struct Unit {
    pub id: Uuid,
    pub owner_id: OwnerId,
    pub health: u32,
    pub hex: Hex,
    pub energy: u32,
    pub age: u32,
    pub body: UnitBody,
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

impl Unit {
    pub fn new(hex: Hex) -> Self {
        Self {
            health: 100,
            hex,
            ..Default::default()
        }
    }

    pub fn max_age(&self) -> u32 {
        ((self.body[UnitPart::Generate] * AGE_PER_GEN_PART) as f32).powf(UNIT_AGE_EXP) as u32
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
        self.body[UnitPart::Ranged]
    }

    pub fn damage(&self) -> u32 {
        self.body[UnitPart::Ranged]
    }

    pub fn attack_cost(&self) -> u32 {
        self.body[UnitPart::Ranged]
    }

    pub fn attack<T>(&self, target: T, intents: &mut Intents)
    where
        T: HasHealth + HasId,
    {
        intents.push(Intent::UnitAttack(intents::UnitAttack {
            unit_id: self.id,
            target_id: target.id(),
        }));
    }

    pub fn attack_checked<T>(&self, target: T, intents: &mut Intents)
    where
        T: HasHealth + HasId,
    {
        // Checks see if the itnent is likely to be converted into an action

        self.attack(target, intents);
    }
}

pub type UnitBody = EnumMap<UnitPart, u32>;

#[derive(enum_map::Enum, Serialize)]
pub enum UnitPart {
    Ranged,
    Harvest,
    Generate,
    Work,
    Battery,
}
