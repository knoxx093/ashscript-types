use std::any::TypeId;

use hecs::serialize::column::{
    deserialize_column, try_serialize, try_serialize_id, DeserializeContext, SerializeContext,
};
use hecs::{Archetype, ColumnBatchBuilder, ColumnBatchType, World};
use postcard::ser_flavors::Slice;
use serde::{Deserialize, Serialize};

use crate::components::body::UnitBody;
use crate::components::energy::Energy;
use crate::components::owner::Owner;
use crate::components::resource::{CoalNode, MineralNode, ResourceNode, UraniumNode};
use crate::components::shield::Shield;
use crate::components::spawn::Spawning;
use crate::components::storage::Storage;
use crate::components::terrain::{Lava, Plain, Terrain, Wall};
use crate::components::tile::Tile;

pub struct WorldWrapper(pub World);

fn serialize_archetypes() {}

// Identifiers for the components we want to include in the serialization process:
#[derive(Serialize, Deserialize)]
enum ComponentId {
    Owner,
    ResourceNode,
    CoalNode,
    MineralNode,
    UraniumNode,
    Shield,
    Spawning,
    Body,
    Energy,
    Storage,
    Terrain,
    Wall,
    Lava,
    Plain,
    Tile,
}

// We need to implement context types for the hecs serialization process:
#[derive(Default)]
struct SaveContextSerialize {}
#[derive(Default)]
struct SaveContextDeserialize {
    components: Vec<ComponentId>,
}

impl DeserializeContext for SaveContextDeserialize {
    fn deserialize_component_ids<'de, A>(&mut self, mut seq: A) -> Result<ColumnBatchType, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        self.components.clear(); // Discard data from the previous archetype
        let mut batch = ColumnBatchType::new();
        while let Some(id) = seq.next_element()? {
            match id {
                ComponentId::Owner => {
                    batch.add::<Owner>();
                }
                ComponentId::ResourceNode => {
                    batch.add::<ResourceNode>();
                }
                ComponentId::CoalNode => {
                    batch.add::<CoalNode>();
                }
                ComponentId::MineralNode => {
                    batch.add::<MineralNode>();
                }
                ComponentId::UraniumNode => {
                    batch.add::<UraniumNode>();
                }
                ComponentId::Shield => {
                    batch.add::<Shield>();
                }
                ComponentId::Spawning => {
                    batch.add::<Spawning>();
                }
                ComponentId::Body => {
                    batch.add::<UnitBody>();
                }
                ComponentId::Energy => {
                    batch.add::<Energy>();
                }
                ComponentId::Storage => {
                    batch.add::<Storage>();
                }
                ComponentId::Terrain => {
                    batch.add::<Terrain>();
                }
                ComponentId::Wall => {
                    batch.add::<Wall>();
                }
                ComponentId::Lava => {
                    batch.add::<Lava>();
                }
                ComponentId::Plain => {
                    batch.add::<Plain>();
                }
                ComponentId::Tile => {
                    batch.add::<Tile>();
                }
            }
            self.components.push(id);
        }
        Ok(batch)
    }

    fn deserialize_components<'de, A>(
        &mut self,
        entity_count: u32,
        mut seq: A,
        batch: &mut ColumnBatchBuilder,
    ) -> Result<(), A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        // Decode component data in the order that the component IDs appeared
        for component in &self.components {
            match *component {
                ComponentId::Owner => {
                    deserialize_column::<Owner, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::ResourceNode => {
                    deserialize_column::<ResourceNode, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::CoalNode => {
                    deserialize_column::<CoalNode, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::MineralNode => {
                    deserialize_column::<MineralNode, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::UraniumNode => {
                    deserialize_column::<UraniumNode, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Shield => {
                    deserialize_column::<Shield, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Spawning => {
                    deserialize_column::<Spawning, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Body => {
                    deserialize_column::<UnitBody, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Energy => {
                    deserialize_column::<Energy, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Storage => {
                    deserialize_column::<Storage, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Terrain => {
                    deserialize_column::<Terrain, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Wall => {
                    deserialize_column::<Wall, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Lava => {
                    deserialize_column::<Lava, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Plain => {
                    deserialize_column::<Plain, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Tile => {
                    deserialize_column::<Tile, _>(entity_count, &mut seq, batch)?;
                }
            }
        }
        Ok(())
    }
}

impl SerializeContext for SaveContextSerialize {
    fn component_count(&self, archetype: &Archetype) -> usize {
        archetype
            .component_types()
            .count()
    }

    fn serialize_component_ids<S: serde::ser::SerializeTuple>(
        &mut self,
        archetype: &Archetype,
        mut out: S,
    ) -> Result<S::Ok, S::Error> {
        try_serialize_id::<Owner, _, _>(archetype, &ComponentId::Owner, &mut out)?;
        try_serialize_id::<ResourceNode, _, _>(archetype, &ComponentId::ResourceNode, &mut out)?;
        try_serialize_id::<CoalNode, _, _>(archetype, &ComponentId::CoalNode, &mut out)?;
        try_serialize_id::<MineralNode, _, _>(archetype, &ComponentId::MineralNode, &mut out)?;
        try_serialize_id::<UraniumNode, _, _>(archetype, &ComponentId::UraniumNode, &mut out)?;
        try_serialize_id::<Shield, _, _>(archetype, &ComponentId::Shield, &mut out)?;
        try_serialize_id::<Spawning, _, _>(archetype, &ComponentId::Spawning, &mut out)?;
        try_serialize_id::<UnitBody, _, _>(archetype, &ComponentId::Body, &mut out)?;
        try_serialize_id::<Energy, _, _>(archetype, &ComponentId::Energy, &mut out)?;
        try_serialize_id::<Storage, _, _>(archetype, &ComponentId::Storage, &mut out)?;
        try_serialize_id::<Terrain, _, _>(archetype, &ComponentId::Terrain, &mut out)?;
        try_serialize_id::<Wall, _, _>(archetype, &ComponentId::Wall, &mut out)?;
        try_serialize_id::<Lava, _, _>(archetype, &ComponentId::Lava, &mut out)?;
        try_serialize_id::<Plain, _, _>(archetype, &ComponentId::Plain, &mut out)?;
        try_serialize_id::<Tile, _, _>(archetype, &ComponentId::Tile, &mut out)?;
        out.end()
    }

    fn serialize_components<S: serde::ser::SerializeTuple>(
        &mut self,
        archetype: &Archetype,
        mut out: S,
    ) -> Result<S::Ok, S::Error> {
        try_serialize::<Owner, _>(archetype, &mut out)?;
        try_serialize::<ResourceNode, _>(archetype, &mut out)?;
        try_serialize::<CoalNode, _>(archetype, &mut out)?;
        try_serialize::<MineralNode, _>(archetype, &mut out)?;
        try_serialize::<UraniumNode, _>(archetype, &mut out)?;
        try_serialize::<Shield, _>(archetype, &mut out)?;
        try_serialize::<Spawning, _>(archetype, &mut out)?;
        try_serialize::<UnitBody, _>(archetype, &mut out)?;
        try_serialize::<Energy, _>(archetype, &mut out)?;
        try_serialize::<Storage, _>(archetype, &mut out)?;
        try_serialize::<Terrain, _>(archetype, &mut out)?;
        try_serialize::<Wall, _>(archetype, &mut out)?;
        try_serialize::<Lava, _>(archetype, &mut out)?;
        try_serialize::<Plain, _>(archetype, &mut out)?;
        try_serialize::<Tile, _>(archetype, &mut out)?;
        out.end()
    }
}

pub fn serialize_world_data(world: &World) -> Vec<u8> {
    let mut buffer_vec: Vec<u8> = vec![];
    let buffer: &mut [u8] = buffer_vec.as_mut_slice();
    let buffer_storage: Slice = Slice::new(buffer);
    let mut serializer = postcard::Serializer { output: buffer_storage };

    let _ = hecs::serialize::column::serialize(world, &mut SaveContextSerialize::default(), &mut serializer);

    println!("buffer slice {:?}", buffer);
    // println!("buffer storage {:#?}", buffer_storage);
    // println!("serializer {:#?}", serializer);
    buffer_vec
}

pub fn deserialize_world_data(
    world_data: Vec<u8>,
) -> Option<World> {

    let mut deserializer = postcard::Deserializer::from_bytes(&world_data);
    hecs::serialize::column::deserialize(&mut SaveContextDeserialize::default(), &mut deserializer).ok()
}

/* impl<'de> Deserialize<'de> for World {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {

        World::deserialize(deserializer)
    }
}

impl Serialize for World {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.serialize(serializer)

    }
} */
