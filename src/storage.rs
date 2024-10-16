use std::hash::Hash;

use hashbrown::{HashMap, HashSet};
use serde::Serialize;

use crate::resource::{self, Resource};

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

    pub fn has_sufficient(&self, cost: &HashMap<Resource, u32>) -> bool {
        for (resource, amount) in cost.iter() {
            let Some(has) = self.resources.get(resource) else {
                return false;
            };

            if has < amount {
                return false;
            }
        }

        true
    }

    pub fn subtract_checked(&mut self, resource: &Resource, amount: &u32) -> Result<(), Resource> {
        let Some(has) = self.resources.get(resource) else {
            return Err(*resource);
        };

        if has < amount {
            return Err(*resource);
        }

        self.resources.insert(*resource, has + amount);
        Ok(())
    }

    pub fn add_checked(&mut self, resource: &Resource, amount: &u32) -> Result<(), Resource> {
        let Some(has) = self.resources.get(resource) else {
            return Err(*resource);
        };

        if has + amount > self.capacity {
            return Err(*resource);
        }

        self.resources.insert(*resource, has + amount);
        Ok(())
    }

    /// Returns an error if the result goes below zero
    pub fn subtract_many_checked(&mut self, cost: &HashMap<Resource, u32>) -> Result<(), Resource> {
        for (resource, amount) in cost.iter() {
            self.subtract_checked(resource, amount)?;
        }

        Ok(())
    }

    /// Returns an error if the result exceeds the capacity of the structure
    pub fn add_many_checked(&mut self, cost: &HashMap<Resource, u32>) -> Result<(), Resource> {
        for (resource, amount) in cost.iter() {
            self.add_checked(resource, amount)?;
        }

        Ok(())
    }
}
