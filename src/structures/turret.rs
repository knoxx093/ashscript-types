use hexx::Hex;

pub struct Turret {
    pub owner_id: u32,
    pub energy: u32,
    pub hex: Hex,
}

impl Turret {
    pub fn attack_cost(&self) -> u32 {
        self.range() + self.damage()
    }
    
    pub fn range(&self) -> u32 {
        1
    }
    
    pub fn damage(&self) -> u32 {
        1
    }
}