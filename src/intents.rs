use hexx::Hex;
use uuid::Uuid;

use crate::{resource::Resource, unit::UnitBody};

// REMINDER: These are player-generated intents

pub type Intents = Vec<Intent>;

pub enum Intent {
    UnitMove(UnitMove),
    UnitAttack(UnitAttack),
    TurretAttack(TurretAttack),
    FactorySpawn(FactorySpawn),
    ResourceTransfer(ResourceTransfer),
}

#[derive(enum_map::Enum, Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum IntentName {
    UnitMove,
    UnitAttack,
    TurretAttack,
    FactorySpawn,
    ResourceTransfer,
}

pub struct UnitMove {
    pub unit_id: Uuid,
    pub from: Hex,
    pub to: Hex,
}

pub struct UnitAttack {
    pub unit_id: Uuid,
    pub target_id: Uuid,
}

pub struct TurretAttack {
    pub turret_id: Uuid,
    pub target_id: Uuid,
}

pub struct FactorySpawn {
    pub factory_id: Uuid,
    pub body: UnitBody,
    pub name: String,
    /// If out hexes are not provided the engine will choose the first empty one in a clockwise direction
    /// If out hexes are provided, sucessfully spawned units will be outputed to the first empty hex 
    pub out: Option<Vec<Hex>>,
}

pub struct ResourceTransfer {
    pub resource: Resource,
    pub amount: u32,
    pub from_id: Uuid,
    pub to_id: Uuid,
}
