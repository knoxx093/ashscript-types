use hexx::Hex;
use uuid::Uuid;

use crate::{structures::{factory::Factory, turret::Turret}, unit::Unit};



pub trait HasHealth {
    fn health(&self) -> u32;
}

pub enum Attackable {
    Unit,
    Turret,
    Factory,
}

pub trait HasId {
    fn id(&self) -> Uuid;
}

pub enum GameObject<'a> {
    Unit(&'a Unit),
    Turret(&'a Turret),
    Factory(&'a Factory),
}

pub enum GameObjectMut<'a> {
    Unit(&'a mut Unit),
    Turret(&'a mut Turret),
    Factory(&'a mut Factory),
}

/// Each type has its own storage inside of a chunk, and cannot share a hex/tile with another of its own type (ex: two units cannot be on the same tile)
pub enum GameObjectKind {
    Unit,
    Turret,
    Factory,
}

pub struct Identifier {
    pub hex: Hex,
    pub kind: GameObjectKind,
}