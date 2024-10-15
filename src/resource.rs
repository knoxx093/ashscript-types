use serde::Serialize;

#[derive(enum_map::Enum, Hash, Eq, PartialEq, Clone, Copy, Debug, Serialize)]
pub enum Resource {
    Coal,
    Minerals,
    Scrap,
    Energy,
    Metal,
}
