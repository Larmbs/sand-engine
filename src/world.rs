use crate::ChunkMesh;

use super::Generator;
use anyhow::Result;
use bincode::{deserialize_from, serialize_into};
use macroquad::color::Color;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Local};

/// Maximum of 256 blocks
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Block {
    Air,
    GrassBlock,
    Dirt,
    Stone,
    Water,
    WaterEdge,
    Sand,
    SandStone,
}
impl Block {
    pub fn color(&self) -> Color {
        match self {
            Block::Air => Color::new(0., 0., 0., 0.),
            Block::GrassBlock => Color::from_hex(0x307a2a),
            Block::Dirt => Color::from_hex(0xba7938),
            Block::Stone => Color::from_hex(0x515357),
            Block::Water | Block::WaterEdge => Color::from_hex(0x4b53eb),
            Block::Sand => Color::from_hex(0xbbc26d),
            Block::SandStone => Color::from_hex(0xe1e897),
        }
    }
}

/// Chunks stores 16x16 blocks
#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chunk {
    #[serde_as(as = "[_; 16*16]")]
    pub blocks: [Block; 16 * 16],
    #[serde(skip_serializing, default = "default_time", skip_deserializing)]
    pub last_used: DateTime<Local>,
}
impl Chunk {
    pub fn new(blocks: [Block; 16 * 16]) -> Self {
        Self {
            blocks,
            last_used: Local::now(),
        }
    }
}

/// Stores 16x16 chunks
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Region {
    pub region_x: i32,
    pub region_y: i32,
    #[serde(skip_serializing, default = "default_time", skip_deserializing)]
    pub last_used: DateTime<Local>,
    #[serde_as(as = "[_; 16*16]")]
    pub chunks: [Option<Chunk>; 16 * 16],
    #[serde_as(as = "[_; 16*16]")]
    #[serde(skip_serializing, default = "default_chunk_meshes", skip_deserializing)]
    pub chunk_meshes: [Option<ChunkMesh>; 16 * 16]
}
fn default_chunk_meshes() -> [Option<ChunkMesh>; 16 * 16] {
    [const { None }; 16 * 16]
}
fn default_time() -> DateTime<Local> {
    Local::now()
}
impl Region {
    pub fn get_chunk(&mut self, gen: &Generator, x: &u8, y: &u8) -> &Chunk {
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
    pub fn get_chunk_mesh(&mut self, gen: &Generator, x: &u8, y: &u8) -> &ChunkMesh {
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
    pub fn load(region_x: &i32, region_y: &i32) -> Self {
        let path = Self::get_region_path(&region_x, &region_y);
        let file = match fs::File::open(path) {
            Ok(file) => file,
            Err(_) => return Self::new_empty(region_x, region_y),
        };
        match deserialize_from(file) {
            Ok(region) => region,
            Err(_) => Self::new_empty(region_x, region_y),
        }
    }
    pub fn save(&self) -> Result<()> {
        let path = Self::get_region_path(&self.region_x, &self.region_y);

        // Serialize the region and save it to the file
        let file = fs::File::create(path)?;
        serialize_into(file, self).unwrap();

        Ok(())
    }
    pub fn new_empty(region_x: &i32, region_y: &i32) -> Self {
        Self {
            region_x: *region_x,
            region_y: *region_y,
            last_used: Local::now(),
            chunks: [const { None }; 16 * 16],
            chunk_meshes: default_chunk_meshes()
        }
    }
}

// /// Represents world meta data file with info on world
// #[derive(Serialize, Deserialize)]
// pub struct World {
//     name: String,
//     seed: u32,
// }
// impl World {
//     pub fn load(world_name: String) -> Result<Self> {
//         let world_path = PathBuf::from("worlds").join(world_name);
//         let reader =
//             fs::File::open(world_path.join("world.json")).context("Failed to find world")?;
//         serde_json::from_reader(reader).context("World file is broken")
//     }
// }
