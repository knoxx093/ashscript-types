use hexx::Hex;

use crate::{id::UUID, resource::Resource, structures::turret::Turret, unit::Unit};

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
    pub unit_id: UUID,
    pub from: Hex,
    pub to: Hex,
}
pub struct UnitAttack {
    pub unit_id: UUID,
    pub target_id: UUID,
    pub damage: u32,
}

pub struct TurretAttack {
    pub turret_id: UUID,
    pub target_id: UUID,
    pub damage: u32,
    pub cost: u32,
}

pub struct FactorySpawn {
    pub factory_id: UUID,
    pub unit_id: u32,
    pub out: Hex,
}

pub struct ResourceTransfer {
    pub resource: Resource,
    pub amount: u32,
    pub from_id: UUID,
    pub to_id: UUID,
}

