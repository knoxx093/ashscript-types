use serde::{Deserialize, Serialize};

#[derive(enum_map::Enum, Hash, Eq, PartialEq, Clone, Copy, Debug, Serialize, Deserialize,)]
pub enum Resource {
    Coal,
    Minerals,
    Scrap,
    Energy,
    Metal,
    Uranium,
}
