use hexx::Hex;

use crate::{id::UUID, resource::Resource, structures::turret::Turret, unit::{Unit, UnitBody}, IdType};

// REMINDER: These are player-generated intents

pub type Intents = Vec<Intent>;

pub enum Intent {
    UnitMove(UnitMove),
    UnitAttack(UnitAttack),
    TurretAttack(TurretAttack),
    FactorySpawn(FactorySpawn),
    ResourceTransfer(ResourceTransfer),
}

pub enum IntentName {
    UnitMove,
    UnitAttack,
    TurretAttack,
    FactorySpawn,
    ResourceTransfer,
}

pub struct UnitMove {
    pub unit_id: UUID,
    pub from: Hex,
    pub to: Hex,
}

pub struct UnitAttack {
    pub unit_id: UUID,
    pub target_id: UUID,
}

pub struct TurretAttack {
    pub turret_id: UUID,
    pub target_id: UUID,
}

pub struct FactorySpawn {
    pub factory_id: UUID,
    pub body: UnitBody,
    pub name: String,
    /// If out hexes are not provided the engine will choose the first empty one in a clockwise direction
    /// If out hexes are provided, sucessfully spawned units will be outputed to the first empty hex 
    pub out: Option<Vec<Hex>>,
}

pub struct ResourceTransfer {
    pub resource: Resource,
    pub amount: u32,
    pub from_id: UUID,
    pub to_id: UUID,
}
