use hashbrown::HashMap;
use hexx::Hex;
use uuid::Uuid;

use crate::{objects::{Attackable, GameObjectKind, HasStorage, WithStorage}, resource::Resource, unit::UnitBody};

// REMINDER: These are intents that the server validates and wants executed

#[derive(Default, Debug)]
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
}

#[derive(Debug)]
pub struct UnitMove {
    pub from: Hex,
    pub to: Hex,
    pub cost: u32,
}

#[derive(Debug)]
pub struct UnitAttack {
    pub attacker_hex: Hex,
    pub target_hex: Hex,
    pub target_kind: Attackable,
    pub cost: u32,
    pub damage: u32,
}

#[derive(Debug)]
pub struct TurretAttack {
    pub turret_hex: Hex,
    pub target_hex: Hex,
    pub target_kind: Attackable,
    pub damage: u32,
    pub cost: u32,
}

#[derive(Debug)]
pub struct FactorySpawnUnit {
    pub factory_hex: Hex,
    pub out: Hex,
    pub body: UnitBody,
    pub name: String,
    pub cost: HashMap<Resource, u32>,
}

#[derive(Debug)]
pub struct UnitSpawnUnit {
    pub parent_hex: Hex,
    pub out: Hex,
    pub body: UnitBody,
    pub name: String,
    pub cost: HashMap<Resource, u32>,
}

#[derive(Debug)]
pub struct ResourceTransfer {
    pub resource: Resource,
    pub amount: u32,
    pub from: Hex,
    pub from_kind: WithStorage,
    pub to_hex: Hex,
    pub to_kind: WithStorage,
}

#[derive(Debug)]
pub struct ObjectDestroyed {
    pub hex: Hex,
    pub kind: GameObjectKind,
}