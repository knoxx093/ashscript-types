use hexx::Hex;

use crate::resource::Resource;

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
    pub unit_id: u32,
    pub from: Hex,
    pub to: Hex,
}

pub struct UnitAttack {
    pub unit_id: u32,
    pub target_id: u32,
    pub damage: u32,
}

pub struct TurretAttack {
    pub turret_id: u32,
    pub target_id: u32,
    pub damage: u32,
    pub cost: u32,
}

pub struct FactorySpawn {
    pub factory_id: u32,
    pub unit_id: u32,
    pub out: Hex,
}

pub struct ResourceTransfer {
    pub resource: Resource,
    pub amount: u32,
    pub from_id: u32,
    pub to_id: u32,
}
