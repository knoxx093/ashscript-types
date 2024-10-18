use enum_map::{enum_map, EnumMap};
use hashbrown::HashSet;
use lazy_static::lazy_static;

use crate::{resource::Resource, unit::UnitPart};

lazy_static! {
    pub static ref RESOURCE_INPUTS: EnumMap<Resource, HashSet<Resource>> = enum_map! {
        Resource::Metal => HashSet::from([Resource::Coal, Resource::Minerals]),
        Resource::Energy => HashSet::new(),
        Resource::Coal => HashSet::new(),
        Resource::Scrap => HashSet::new(),
        Resource::Minerals => HashSet::new(),
        Resource::Uranium => HashSet::new(),
    };
    pub static ref UNIT_PART_WEIGHTS: EnumMap<UnitPart, u32> = enum_map! {
        UnitPart::Ranged => 5,
        UnitPart::Generate => 2,
        UnitPart::Battery => 4,
        UnitPart::Harvest => 2,
        _ => 1,
    };
    pub static ref UNIT_PART_COSTS: EnumMap<UnitPart, (Resource, u32)> = enum_map! {
        UnitPart::Ranged => (Resource::Metal, 8),
        UnitPart::Generate => (Resource::Metal, 4),
        UnitPart::Battery => (Resource::Metal, 2),
        UnitPart::Harvest => (Resource::Metal, 3),
        _ => (Resource::Metal, 1),
    };
}

pub enum IntentReturnCode {
    InsufficientResources,
    WrongArgument,
    OutOfRange,
}