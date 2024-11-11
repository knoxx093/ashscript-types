use serde::{Deserialize, Serialize};

use crate::objects::GameObjectKind;

use super::body::UnitBody;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Health(pub u32);

impl Default for Health {
    fn default() -> Self {
        Self(100)
    }
}

impl Health {
    pub fn from_kind(kind: GameObjectKind) -> Self {
        match kind {
            GameObjectKind::Unit => Self(10),
            GameObjectKind::Turret => Self(100),
            _ => Self::default(),
        }
    }

    pub fn for_unit(body: &UnitBody) -> Self {
        Self(body.health_without_shields())
    }

    pub fn current_with_shields() {
        todo!()
    }

    pub fn max_with_shields() {
        todo!()
    }   
}