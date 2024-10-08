use hexx::Hex;

use crate::resource::Resource;
    pub struct MoveAction {
        pub unit_id: u32,
        pub from: Hex,
        pub to: Hex,
    }

    pub struct AttackAction {
        pub unit_id: u32,
        pub target_id: u32,
        pub from: Hex,
        pub damage: u32,
    }

    pub struct FactorySpawn {
        pub factory_id: u32,
        pub unit_id: u32,
        pub out: Hex,
    }

    pub struct TurretAttack {
        pub turret_id: u32,
        pub target_id: u32,
        pub damge: u32,
    }

    pub struct ResourceAttack {
        pub resource: Resource,
        pub amount: u32,
        pub from_id: u32,
        pub to_id: u32,
    }