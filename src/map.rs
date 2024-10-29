use hexx::Hex;
use serde::{Deserialize, Serialize};

use crate::{chunk::{Chunk, Chunks}, constants::map::CHUNK_SIZE, structures::{factory::Factory, turret::Turret}, terrain::Terrain, unit::Unit};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Map {
    pub chunks: Chunks,
    pub data: MapData,
}

impl Map {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn chunk_at(&self, hex: &Hex) -> Option<&Chunk> {
        let chunk_hex = hex.to_lower_res(CHUNK_SIZE);
        self.chunks.get(&chunk_hex)
    }

    pub fn chunk_at_mut(&mut self, hex: &Hex) -> Option<&mut Chunk> {
        let chunk_hex = hex.to_lower_res(CHUNK_SIZE);
        self.chunks.get_mut(&chunk_hex)
    }

    pub fn terrain_at(&self, hex: &Hex) -> Option<&Terrain> {
        let chunk = self.chunk_at(hex)?;
        chunk.terrain.get(hex)
    }

    pub fn unit_at(&self, hex: &Hex) -> Option<&Unit> {
        let chunk = self.chunk_at(hex)?;
        chunk.units.get(hex)
    }

    pub fn unit_at_mut(&mut self, hex: &Hex) -> Option<&mut Unit> {
        let chunk = self.chunk_at_mut(hex)?;
        chunk.units.get_mut(hex)
    }

    pub fn factory_at(&self, hex: &Hex) -> Option<&Factory> {
        let chunk = self.chunk_at(hex)?;
        chunk.factories.get(hex)
    }

    pub fn factory_at_mut(&mut self, hex: &Hex) -> Option<&mut Factory> {
        let chunk = self.chunk_at_mut(hex)?;
        chunk.factories.get_mut(hex)
    }

    pub fn turret_at(&self, hex: &Hex) -> Option<&Turret> {
        let chunk = self.chunk_at(hex)?;
        chunk.turrets.get(hex)
    }

    pub fn turret_at_mut(&mut self, hex: &Hex) -> Option<&mut Turret> {
        let chunk = self.chunk_at_mut(hex)?;
        chunk.turrets.get_mut(hex)
    }

    /* pub fn game_object(&self, identifier: &Identifier) -> Option<GameObject> {

        match identifier.kind {
            GameObjectKind::Unit => {
                let object = self.unit_at(&identifier.hex)?;
                Some(GameObject::Unit(object))
            },
            GameObjectKind::Turret => {
                let object = self.turret_at(&identifier.hex)?;
                Some(GameObject::Turret(object))
            }
            GameObjectKind::Factory => {
                let object = self.factory_at(&identifier.hex)?;
                Some(GameObject::Factory(object))
            }
            _ => None,
        }
    }

    pub fn game_object_mut<T>(&mut self, identifier: &Identifier) -> Option<GameObjectMut> {
        match identifier.kind {
            GameObjectKind::Unit => {
                let object = self.unit_at_mut(&identifier.hex)?;
                Some(GameObjectMut::Unit(object))
            }
            GameObjectKind::Turret => {
                let object = self.turret_at_mut(&identifier.hex)?;
                Some(GameObjectMut::Turret(object))
            }
            GameObjectKind::Factory => {
                let object = self.factory_at_mut(&identifier.hex)?;
                Some(GameObjectMut::Factory(object))
            }
            _ => None,
        }
    } */
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct MapData {
    pub radius: u32,
}

impl MapData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}