use uuid::Uuid;



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