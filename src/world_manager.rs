use super::{gen::WorldGenerator, ChunkMesh, Region};
use crate::blocks::Block;
use chrono::{Duration, Local};
use std::collections::HashMap;

pub struct WorldManager {
    gen: WorldGenerator,
    regions: HashMap<(i32, i32), Region>,
}

impl WorldManager {
    pub fn new(seed: u32) -> Self {
        Self {
            gen: WorldGenerator::new(seed),
            regions: HashMap::new(),
        }
    }

    pub fn get_chunk_mesh(
        &mut self,
        region_x: &i32,
        region_y: &i32,
        regional_chunk_x: &u8,
        regional_chunk_y: &u8,
    ) -> &ChunkMesh {
        if !self.regions.contains_key(&(*region_x, *region_y)) {
            self.regions
                .insert((*region_x, *region_y), Region::load(region_x, region_y));
        }

        let region = self.regions.get_mut(&(*region_x, *region_y)).unwrap();
        region.get_chunk_mesh(&self.gen, regional_chunk_x, regional_chunk_y)
    }

    pub fn clean(&mut self) {
        let now = Local::now();
        let keys_to_remove: Vec<_> = self
            .regions
            .iter()
            .filter_map(|(key, region)| {
                if now - region.last_used > Duration::seconds(2) {
                    Some(key.clone())
                } else {
                    None
                }
            })
            .collect();

        for key in keys_to_remove {
            self.regions.remove(&key);
        }
    }

    pub fn get_region_count(&self) -> usize {
        self.regions.len()
    }

    pub fn get_block(&mut self, world_x: &i64, world_y: &i64) -> &Block {
        let (region_x, region_y) = conversion::get_region_cords(world_x, world_y);
        if !self.regions.contains_key(&(region_x, region_y)) {
            self.regions
                .insert((region_x, region_y), Region::load(&region_x, &region_y));
        }
        let (chunk_x, chunk_y) = conversion::get_region_chunk_cords(world_x, world_y);
        let (local_x, local_y) = conversion::get_local_chunk_cords(world_x, world_y);
        let region = self.regions.get_mut(&(region_x, region_y)).unwrap();
        region.get_block(&self.gen, &chunk_x, &chunk_y, &local_x, &local_y)
    }
}

pub mod conversion {
    pub fn get_region_cords(world_x: &i64, world_y: &i64) -> (i32, i32) {
        ((world_x >> 8) as i32, (world_y >> 8) as i32)
    }

    pub fn get_region_chunk_cords(world_x: &i64, world_y: &i64) -> (u8, u8) {
        (
            ((world_x >> 4) & 0b1111) as u8,
            ((world_y >> 4) & 0b1111) as u8,
        )
    }

    pub fn get_chunk_world_cords(world_x: &i64, world_y: &i64) -> (i64, i64) {
        (world_x & !0b1111, world_y & !0b1111)
    }

    pub fn get_local_chunk_cords(world_x: &i64, world_y: &i64) -> (u8, u8) {
        ((world_x & 0b1111) as u8, (world_y & 0b1111) as u8)
    }
}
