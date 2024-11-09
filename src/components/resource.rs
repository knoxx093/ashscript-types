use serde::{Deserialize, Serialize};


#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ResourceNode {
    pub amount: u32
}

impl ResourceNode {
    pub fn new(amount: u32) -> Self {
        Self {
            amount
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct CoalNode;
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct MineralNode;
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct UraniumNode;