use hexx::Hex;
use uuid::Uuid;

use crate::{objects::Attackable, resource::Resource};

// REMINDER: These are intents that the server validates and wants executed

pub type Actions = Vec<Action>;

pub enum Action {
    UnitMove(UnitMove),
    UnitAttack(UnitAttack),
    TurretAttack(TurretAttack),
    FactorySpawnUnit(FactorySpawnUnit),
    UnitSpawnUnit(UnitSpawnUnit),
    ResourceTransfer(ResourceTransfer),
}

#[derive(Default)]
pub struct ActionsByKind {
    pub unit_move: Vec<UnitMove>,
    pub unit_attack: Vec<UnitAttack>,
    pub turret_attack: Vec<TurretAttack>,
    pub factory_spawn_unit: Vec<FactorySpawnUnit>,
    pub unit_spawn_unit: Vec<UnitSpawnUnit>,
    pub resource_transfer: Vec<ResourceTransfer>,
}

impl ActionsByKind {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Add an action of a non-specified kind
    fn add_action(&mut self, intent: Action) {
        match intent {
            Action::UnitMove(unit_move) => self.unit_move.push(unit_move),
            Action::UnitAttack(unit_attack) => self.unit_attack.push(unit_attack),
            Action::TurretAttack(turret_attack) => self.turret_attack.push(turret_attack),
            Action::FactorySpawnUnit(factory_spawn) => self.factory_spawn_unit.push(factory_spawn),
            Action::UnitSpawnUnit(unit_spawn_unit) => self.unit_spawn_unit.push(unit_spawn_unit),
            Action::ResourceTransfer(resource_transfer) => self.resource_transfer.push(resource_transfer),
        }
    }
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
    pub turret_hex: Hex,
    pub target_hex: Hex,
    pub target_kind: Attackable,
    pub damage: u32,
    pub cost: u32,
}

pub struct FactorySpawnUnit {
    pub factory_id: Uuid,
    pub unit_id: u32,
    pub out: Hex,
}

pub struct UnitSpawnUnit {
    pub parent_id: Uuid,
    pub child_id: Uuid,
    pub out: Hex,
}

pub struct ResourceTransfer {
    pub resource: Resource,
    pub amount: u32,
    pub from_id: Uuid,
    pub to_id: Uuid,
}

pub struct ObjectDestroyed {
    pub id: Uuid,
}