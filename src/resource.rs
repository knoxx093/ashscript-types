use enum_map::{enum_map, EnumMap};
use hashbrown::HashSet;

#[derive(enum_map::Enum, Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Resource {
    Coal,
    Minerals,
    Scrap, // unsure if this should be a separate resource from metal
    Energy,
    Metal,
}
