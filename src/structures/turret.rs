use hexx::Hex;

pub struct Turret {
    pub owner_id: u32,
    pub energy: u32,
    pub hex: Hex,
}

impl Turret {
    pub fn turret_attack_cost(&self) -> u32 {
        self.turret_range() + self.turret_damage()
    }
    
    pub fn turret_range(&self) -> u32 {
        1
    }
    
    pub fn turret_damage(&self) -> u32 {
        1
    }
}