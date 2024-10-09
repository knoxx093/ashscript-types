use enum_map::{enum_map, EnumMap};
use glam::Vec2;
use hashbrown::HashSet;
use hexx::{HexLayout, HexOrientation};
use lazy_static::lazy_static;

use crate::{resource::Resource, unit::UnitPart};

pub const HEX_SIZE: Vec2 = Vec2::splat(64.0);
pub const CHUNK_SIZE: u32 = 5;
pub const HEX_LAYOUT: HexLayout = HexLayout {
    hex_size: HEX_SIZE,
    orientation: HexOrientation::Pointy,
    origin: Vec2::new(0., 0.),
    invert_x: false,
    invert_y: false,
};

pub const UNIT_AGE_EXP: f32 = 0.4;
pub const AGE_PER_GEN_PART: u32 = 10;
pub const UNIT_BASE_AGE: u32 = 300;

lazy_static! {
    pub static ref RESOURCE_INPUTS: EnumMap<Resource, HashSet<Resource>> = enum_map! {
        Resource::Metal => HashSet::from([Resource::Coal, Resource::Minerals]),
        Resource::Energy => HashSet::new(),
        Resource::Coal => HashSet::new(),
        Resource::Scrap => HashSet::new(),
        Resource::Minerals => HashSet::new(),
    };
    pub static ref UNIT_PART_WEIGHTS: EnumMap<UnitPart, u32> = enum_map! {
        UnitPart::Ranged => 5,
        UnitPart::Generate => 2,
        UnitPart::Battery => 4,
        UnitPart::Harvest => 2,
        _ => 1,
    };
}