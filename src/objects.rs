use enum_dispatch::enum_dispatch;
use enum_iterator::Sequence;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    structures::{factory::Factory, turret::Turret},
    unit::Unit,
};

/// Each type has its own storage inside of a chunk, and cannot share a hex/tile with another of its own type (ex: two units cannot be on the same tile)
/// Not exactly sure if this is useful anywhere
#[derive(Serialize, Deserialize, Clone, Copy, Default, Eq, PartialEq, Hash, Debug, Sequence)]
pub enum GameObjectKind {
    Unit,
    Turret,
    Factory,
    Distributor,
    Assembler,
    #[default]
    Unknown,
}
