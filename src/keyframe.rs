use serde::{Deserialize, Serialize};

use crate::{global::Global, map::Map};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct KeyFrame {
    pub map: Map,
    pub global: Global,
}