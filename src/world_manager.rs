use super::{Block, ChunkMesh, Generator, World};
use std::collections::HashMap;


/// When trying to load a chunk here is the order of operations
/// Check if we have it already loaded in region
/// if the region doesn't contain it then we generate
/// If it is within a diffrent region that is not loaded load that region
/// 


/// Responsible for loading world, editing world, and saving it.
/// It tracts objects usage and automatically unloads them if not in use
/// It loads chunks when asked but it hides region functionality
pub struct WorldManager {
    // Object which contains world gen data
    world: World,
    gen: Generator,
    // Hashmap holds loaded chunk meshes
    meshes: HashMap<(i64, i64), ChunkMesh>,
}
/// These are the public functions open to other objects
impl WorldManager {
    
    pub fn get_chunk(&mut self, x: &i64, y: &i64) -> &ChunkMesh {
        match self.meshes.get(&(*x, *y)) {
            Some(mesh) => {
                // Some code goes here to mark the chunk as used recently
                mesh
            },
            None => {
                // Load the chunk into memory if not already loaded
                todo!()
            }
        }
    }
    pub fn set_block(&mut self, x: i64, y: i64, block: Block) {
        todo!()
    }
}
