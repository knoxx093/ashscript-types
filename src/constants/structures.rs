use enum_map::EnumMap;
use hashbrown::HashSet;
use lazy_static::lazy_static;

use crate::objects::GameObjectKind;

lazy_static! {
    pub static ref IMPASSIBLE_GAME_OBJECTS: HashSet<GameObjectKind> = [
        GameObjectKind::Turret,
        GameObjectKind::Factory,
        GameObjectKind::Unit
    ]
    .iter()
    .cloned()
    .collect::<HashSet<GameObjectKind>>();
}
