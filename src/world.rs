use super::Generator;
use anyhow::Context;
use anyhow::Result;
use bincode::{deserialize_from, serialize_into};
use macroquad::color::Color;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::PathBuf;

/// Maximum of 256 blocks
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
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
#[derive(Serialize, Deserialize, Clone)]
pub struct Chunk {
    pub active: bool,
    pub blocks: Vec<Block>,
}

/// Stores 16x16 chunks
#[derive(Serialize, Deserialize)]
pub struct Region {
    pub region_x: i32,
    pub region_y: i32,
    pub chunks: Vec<Option<Chunk>>,
}
impl Region {
    pub fn new(region_x: i32, region_y: i32, seed: u32) -> Self {
        let gen = Generator::new(seed);
        Region {
            region_x,
            region_y,
            chunks: (0u8..=255u8)
                .into_iter()
                .map(|i| Some(gen.gen_chunk(region_x, region_y, i % 16, i / 16)))
                .collect(),
        }
    }
    pub fn get_chunk(&self, x: &u8, y: &u8) -> &Option<Chunk> {
        assert!(x < &16 && y < &16, "That is outside this region");
        &self.chunks[(x + y * 16) as usize]
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
            chunks: Vec::from([const { None }; 16 * 16]),
        }
    }
}

/// Represents world meta data file with info on world
#[derive(Serialize, Deserialize)]
pub struct World {
    name: String,
    seed: u32,
}
impl World {
    pub fn load(world_name: String) -> Result<Self> {
        let world_path = PathBuf::from("worlds").join(world_name);
        let reader =
            fs::File::open(world_path.join("world.json")).context("Failed to find world")?;
        serde_json::from_reader(reader).context("World file is broken")
    }
}
