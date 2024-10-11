use hexx::Hex;

use crate::{id_type::{Attackable, HasStorage, Id}, resource::Resource, structures::turret::Turret, unit::{Unit, UnitBody}, IdType};

// REMINDER: These are player-generated intents

pub type Intents = Vec<Intent>;

pub enum Intent {
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
    pub target_id: dyn Attackable,
}

pub struct TurretAttack {
    pub turret_id: Id<Turret>,
    pub target_id: dyn Attackable,
}

pub struct FactorySpawn {
    pub factory_id: Id<FactorySpawn>,
    pub body: UnitBody,
    pub name: String,
    /// If out hexes are not provided the engine will choose the first empty one in a clockwise direction
    /// If out hexes are provided, sucessfully spawned units will be outputed to the first empty hex 
    pub out: Option<Vec<Hex>>,
}

pub struct ResourceTransfer {
    pub resource: Resource,
    pub amount: u32,
    pub from_id: Box<dyn HasStorage>,
    pub to_id: Box<dyn HasStorage>,
}
