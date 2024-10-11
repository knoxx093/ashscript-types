use hexx::Hex;

use crate::{id_type::{Attackable, HasStorage, Id}, resource::Resource, structures::turret::Turret, unit::Unit};

// REMINDER: These are intents that the server validates and wants executed

pub type Actions = Vec<Action>;

pub enum Action {
    UnitMove(Box<UnitMove>),
    UnitAttack(Box<UnitAttack>),
    TurretAttack(Box<TurretAttack>),
    FactorySpawn(Box<FactorySpawn>),
    ResourceTransfer(Box<ResourceTransfer>),
}

pub struct UnitMove {
    pub unit_id: Id<Unit>,
    pub from: Hex,
    pub to: Hex,
}

pub struct UnitAttack {
    pub unit_id: Id<Unit>,
    pub target_id: Box<dyn Attackable>,
    pub damage: u32,
}

pub struct TurretAttack {
    pub turret_id: Id<Turret>,
    pub target_id: Box<dyn Attackable>,
    pub damage: u32,
    pub cost: u32,
}

pub struct FactorySpawn {
    pub factory_id: Id<FactorySpawn>,
    pub unit_id: u32,
    pub out: Hex,
}

pub struct ResourceTransfer {
    pub resource: Resource,
    pub amount: u32,
    pub from_id: Box<dyn HasStorage>,
    pub to_id: Box<dyn HasStorage>,
}
