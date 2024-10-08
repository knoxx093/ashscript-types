#[derive(enum_map::Enum, Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Resource {
    Coal,
    Minerals,
    Scrap,
    Energy,
    Metal,
}