use uuid::Uuid;

use crate::objects::GameObjectKind;

pub struct GameObject {
    id: Uuid,
    kind: GameObjectKind,
}