use super::{Block, ChunkMesh, Generator, World, Region};
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
    world: World,
    gen: Generator,
    // Hashmap of loaded regions
    regions: HashMap<(i32, i32), Region>,
    // Hashmap holds loaded chunk meshes
    meshes: HashMap<(i64, i64), ChunkMesh>,
}
/// These are the public functions open to other objects
impl WorldManager {
    pub fn get_chunk_mesh(&mut self, world_x: &i64, world_y: &i64) -> &ChunkMesh {
        match self.meshes.get(&(*world_x, *world_y)) {
            Some(mesh) => {
                mesh
            }
            None => {
                match self.regions.get(&Self::get_region_cords(world_x, world_y)) {
                    Some(region) => {
                        todo!() // Try finding chunk within region and create mesh
                    },
                    None => {
                        todo!() // Must load a new region
                    },
                }
            }
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
}
