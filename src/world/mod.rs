pub mod blocks;
pub mod chunks;
use blocks::Block;
use chunks::Chunk;
pub mod gen;
use gen::Generator;


use super::ChunkMesh;
use anyhow::Result;
use chrono::{DateTime, Local};

/* Serialization */
use bincode::{deserialize_from, serialize_into};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use std::{fs, path::PathBuf};


#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Region {
    pub region_x: i32,
    pub region_y: i32,
    #[serde(skip_serializing, default = "Local::now", skip_deserializing)]
    pub last_used: DateTime<Local>,
    #[serde_as(as = "[_; 16*16]")]
    pub chunks: [Option<Chunk>; 16 * 16],
    #[serde_as(as = "[_; 16*16]")]
    #[serde(skip_serializing, default = "default_chunk_meshes", skip_deserializing)]
    pub chunk_meshes: [Option<ChunkMesh>; 16 * 16],
}
fn default_chunk_meshes() -> [Option<ChunkMesh>; 16 * 16] {
    [const { None }; 16 * 16]
}
impl Region {
    pub fn get_chunk(&mut self, gen: &impl Generator, x: &u8, y: &u8) -> &Chunk {
        assert!(x < &16 && y < &16, "That is outside this region");

        self.last_used = Local::now();
        let index = (x + y * 16) as usize;

        if self.chunks[index].is_none() {
            let chunk = gen.gen_chunk(&self.region_x, &self.region_y, x, y);
            self.chunks[index] = Some(chunk);
        }
        self.chunks[index].as_mut().unwrap().last_used = Local::now();
        self.chunks[index].as_ref().unwrap()
    }
    pub fn get_chunk_mesh(&mut self, gen: &impl Generator, x: &u8, y: &u8) -> &ChunkMesh {
        assert!(x < &16 && y < &16, "That is outside this region");

        self.last_used = Local::now();
        let index = (x + y * 16) as usize;

        if self.chunk_meshes[index].is_none() {
            let mesh = ChunkMesh::greedy_mesh(self.get_chunk(gen, x, y));
            self.chunk_meshes[index] = Some(mesh);
        }

        self.chunks[index].as_mut().unwrap().last_used = Local::now();
        self.chunk_meshes[index].as_ref().unwrap()
    }
    pub fn get_region_path(region_x: &i32, region_y: &i32) -> PathBuf {
        PathBuf::from(format!("world_file/regions/{region_x}.{region_y}.rf"))
    }
    pub fn new_empty(region_x: &i32, region_y: &i32) -> Self {
        Self {
            region_x: *region_x,
            region_y: *region_y,
            last_used: Local::now(),
            chunks: [const { None }; 16 * 16],
            chunk_meshes: default_chunk_meshes(),
        }
    }
    pub fn get_block(&mut self, gen: &impl Generator, chunk_x: &u8, chunk_y: &u8, x: &u8, y: &u8) -> &Block {
        assert!(chunk_x < &16 && chunk_y < &16, "That is outside this region");
        assert!(x < &16 && y < &16, "That is outside the chunk");

        // Update last used timestamp for the region
        self.last_used = Local::now();

        // Calculate the chunk index in the region
        let chunk_index = (chunk_x + chunk_y * 16) as usize;

        // Ensure the chunk exists
        if self.chunks[chunk_index].is_none() {
            let chunk = gen.gen_chunk(&self.region_x, &self.region_y, chunk_x, chunk_y);
            self.chunks[chunk_index] = Some(chunk);
        }

        // Update the last used timestamp for the chunk
        self.chunks[chunk_index].as_mut().unwrap().last_used = Local::now();

        // Retrieve the block from the chunk
        &self.chunks[chunk_index].as_ref().unwrap().blocks[(x + 16 * y) as usize]
    }
}
impl Region {
    /// Loads region from save file if it does not work then create empty one
    pub fn load(region_x: &i32, region_y: &i32) -> Self {
        let path = Self::get_region_path(&region_x, &region_y);

        fs::File::open(&path)
            .map_err(|_| ())
            .and_then(|file| deserialize_from(file).map_err(|_| ()))
            .unwrap_or_else(|_| Self::new_empty(region_x, region_y))
    }
    /// Saves region into save file
    pub fn save(&self) -> Result<()> {
        let path = Self::get_region_path(&self.region_x, &self.region_y);
        fs::File::create(&path).and_then(|file| {
            serialize_into(file, self)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        })?;
        Ok(())
    }
}
