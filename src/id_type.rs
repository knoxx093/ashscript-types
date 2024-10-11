use std::marker::PhantomData;

pub struct Id<T> {
    phantom: PhantomData<fn() -> T>
}

pub trait Attackable {}
pub trait HasStorage {}