use hexx::Hex;
use uuid::Uuid;

use crate::{resource::Resource, structures::turret::Turret, unit::Unit};

// REMINDER: These are intents that the server validates and wants executed

pub type Actions = Vec<Action>;

pub enum Action {
    UnitMove(UnitMove),
    UnitAttack(UnitAttack),
    TurretAttack(TurretAttack),
    FactorySpawn(FactorySpawn),
    ResourceTransfer(ResourceTransfer),
}

pub struct UnitMove {
    pub unit_id: Uuid,
    pub from: Hex,
    pub to: Hex,
}
pub struct UnitAttack {
    pub unit_id: Uuid,
    pub target_id: Uuid,
    pub damage: u32,
}

pub struct TurretAttack {
    pub turret_id: Uuid,
    pub target_id: Uuid,
    pub damage: u32,
    pub cost: u32,
}

pub struct FactorySpawn {
    pub factory_id: Uuid,
    pub unit_id: u32,
    pub out: Hex,
}

pub struct ResourceTransfer {
    pub resource: Resource,
    pub amount: u32,
    pub from_id: Uuid,
    pub to_id: Uuid,
}

