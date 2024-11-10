use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Health(pub u32);

impl Default for Health {
    fn default() -> Self {
        Self(100)
    }
}