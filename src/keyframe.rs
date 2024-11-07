use serde::{Deserialize, Serialize};

use crate::{actions::{self, ActionsByKind}, global::Global, map::Map};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct KeyFrame {
    pub map: Map,
    pub global: Global,
    pub actions: ActionsByKind,
}

impl KeyFrame {
    pub fn from_existing(map: Map, global: Global, actions: ActionsByKind) -> Self {
        Self {
            map,
            global,
            actions,
        }
    }
}