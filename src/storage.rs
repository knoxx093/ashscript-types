use hashbrown::{HashMap, HashSet};
use serde::Serialize;

use crate::resource::Resource;

#[derive(Default, Serialize, Clone)]
pub struct Storage {
    pub resources: HashMap<Resource, u32>,
    #[serde(skip)]
    pub future_resources: HashMap<Resource, u32>,
    /// resources that are allowed to be inserted into the structure. If none, accept all resources
    pub allowed_inputs: Option<HashSet<Resource>>,
    pub capacity: u32,
}   

impl Storage {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}