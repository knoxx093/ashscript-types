use hexx::Hex;
use uuid::Uuid;

use crate::{objects::{Attackable}, resource::Resource, unit::UnitBody};

// REMINDER: These are player-generated intents

pub type Intents = Vec<Intent>;

pub enum Intent {
    UnitMove(UnitMove),
    UnitAttack(UnitAttack),
    TurretAttack(TurretAttack),
    FactorySpawn(FactorySpawnUnit),
    UnitSpawnUnit(UnitSpawnUnit),
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


/// A unit moving from one hex to another
pub struct UnitMove {
    pub from: Hex,
    pub to: Hex,
}

/// A unit attacking an attackable target
pub struct UnitAttack {
    pub attacker_hex: Hex,
    pub target_hex: Hex,
}

/// A turret attacking an attackable target
pub struct TurretAttack {
    pub turret_hex: Hex,
    pub target_hex: Hex,
}

/// The spawning of a unit from a factory
pub struct FactorySpawnUnit {
    pub factory_hex: Hex,
    pub body: UnitBody,
    pub name: String,
    /// If out hexes are not provided the engine will choose the first empty one in a clockwise direction
    /// If out hexes are provided, sucessfully spawned units will be outputed to the first empty hex 
    pub out: Option<Vec<Hex>>,
}

/// The spawning of a unit from a unit
pub struct UnitSpawnUnit {
    pub unit_hex: Hex,
    pub body: UnitBody,
    pub name: String,
    /// If out hexes are not provided the engine will choose the first empty one in a clockwise direction
    /// If out hexes are provided, sucessfully spawned units will be outputed to the first empty hex 
    pub out: Option<Vec<Hex>>,
}

/// A resource transfer from one storable object to another
pub struct ResourceTransfer {
    pub resource: Resource,
    pub amount: u32,
    pub from_hex: Hex,
    pub to_hex: Hex,
}
