use chrono::{Duration, Local, TimeDelta};

use super::{Block, ChunkMesh, Generator, Region};
use std::collections::HashMap;

/// When trying to load a chunk here is the order of operations
/// Check if we have it already loaded in region
/// if the region doesn't contain it then we generate
/// If it is within a different region that is not loaded load that region
///

/// Responsible for loading world, editing world, and saving it.
/// It tracts objects usage and automatically unloads them if not in use
/// It loads chunks when asked but it hides region functionality
pub struct WorldManager {
    // Object which contains world gen data
    gen: Generator,
    // Hashmap of loaded regions
    regions: HashMap<(i32, i32), Region>,
}
/// These are the public functions open to other objects
impl WorldManager {
    pub fn new(seed: u32) -> Self {
        Self {
            gen: Generator::new(seed),
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
            self.regions.insert((*region_x, *region_y), Region::load(region_x, region_y));
        }
    
        let region = self.regions.get_mut(&(*region_x, *region_y)).unwrap();
        region.get_chunk_mesh(&self.gen, regional_chunk_x, regional_chunk_y)
    }
    /// Cleans out all regions that have not been used in a while
    pub fn clean(&mut self) {
        let now = Local::now();
        let keys_to_remove: Vec<_> = self.regions.iter()
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
    /// Returns the regional coordinates from world ones
    pub fn get_region_cords(world_x: &i64, world_y: &i64) -> (i32, i32) {
        ((world_x >> 8) as i32, (world_y >> 8) as i32)
    }
    /// Returns the regional chunk coordinates from world ones
    pub fn get_region_chunk_cords(world_x: &i64, world_y: &i64) -> (u8, u8) {
        (
            ((world_x >> 4) & 0b1111) as u8,
            ((world_y >> 4) & 0b1111) as u8,
        )
    }
    pub fn get_chunk_world_cords(world_x: &i64, world_y: &i64) -> (i64, i64) {
        (world_x & !0b1111, world_y & !0b1111)
    }
}
