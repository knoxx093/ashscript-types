use enum_dispatch::enum_dispatch;
use hexx::Hex;
use serde::Serialize;
use uuid::Uuid;

use crate::{storage::Storage, structures::{factory::Factory, turret::Turret}, unit::Unit};

pub trait HasHex {
    fn hex(&self) -> Hex;
}

pub trait HasHealth {
    fn health(&self) -> u32;
}

#[enum_dispatch(HasHealth)]
pub enum Attackable {
    Unit,
    Turret,
    Factory,
}

pub trait HasId {
    fn id(&self) -> Uuid;
}

pub trait HasStorage {
    fn storage(&self) -> &Storage;
}

/* pub enum GameObject<'a> {
    Unit(&'a Unit),
    Turret(&'a Turret),
    Factory(&'a Factory),
}

pub enum GameObjectMut<'a> {
    Unit(&'a mut Unit),
    Turret(&'a mut Turret),
    Factory(&'a mut Factory),
} */

/// Each type has its own storage inside of a chunk, and cannot share a hex/tile with another of its own type (ex: two units cannot be on the same tile)
/// Not exactly sure if this is useful anywhere
#[derive(Serialize, Clone, Copy, Default)]
pub enum GameObjectKind {
    Unit,
    Turret,
    Factory,
    #[default]
    Unknown,
}

/* pub struct Identifier {
    pub hex: Hex,
    pub kind: GameObjectKind,
} */