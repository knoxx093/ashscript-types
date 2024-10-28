use serde::{Deserialize, Serialize};

use crate::{global::Global, map::Map};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct KeyFrame {
    pub map: Map,
    pub global: Global,
}

impl KeyFrame {
    pub fn from_existing(map: Map, global: Global) -> Self {
        Self {
            map,
            global,
        }
    }
}