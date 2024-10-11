use enum_map::{enum_map, EnumMap};
use hashbrown::HashSet;

#[derive(enum_map::Enum, Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Resource {
    Coal,
    Minerals,
    // Lets make this in a factory instead, makes more sense for
    // a user to mine metal, then put it in a factory for scrap
    // then they can later refine it for better resources. - Jesse
    Scrap, // unsure if this should be a separate resource from metal
    Energy,
    Metal,
}
