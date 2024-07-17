use super::{Block, ChunkMesh, Generator, World};
use std::collections::HashMap;

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
    pub fn load_chunk(&mut self, x: i64, y: i64) {
        todo!()
    }
    /// Returns a hashmap of all meshes loaded
    pub fn get_chunks(&self) -> &HashMap<(i64, i64), ChunkMesh> {
        &self.meshes
    }
    pub fn set_block(&mut self, x: i64, y: i64, block: Block) {
        todo!()
    }
}
