use anyhow::Context;
use bincode::{deserialize_from, serialize_into};
use macroquad::color::Color;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;
use super::Generator;
use serde_json;
use anyhow::Result;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Block {
    AIR,
    GRASS,
    DIRT,
    STONE,
    WATER,
    WaterEdge,
    SAND,
}
impl Block {
    pub fn color(&self) -> Color {
        match self {
            Block::AIR => Color::new(0., 0., 0., 0.),
            Block::GRASS => Color::from_hex(0x307a2a),
            Block::DIRT => Color::from_hex(0xba7938),
            Block::STONE => Color::from_hex(0x515357),
            Block::WATER | Block::WaterEdge => Color::from_hex(0x4b53eb),
            Block::SAND => Color::from_hex(0xbbc26d),
        }
    }
}

/// Stores 16x16 chunks
#[derive(Serialize, Deserialize)]
pub struct Region {
    pub region_x: i32,
    pub region_y: i32,
    pub modified: bool,
    // Cannot enforce Vec size because serde is weird, but this will hold 16x16 blocks
    // Each entry carries bool determining if it is new
    pub chunks: Vec<Option<(bool, Chunk)>>,
}
impl Region {
    pub fn new(region_x: i32, region_y: i32, seed: u32) -> Self {
        let gen = Generator::new(seed);
        Region {
            region_x,
            region_y,
            modified: true,
            chunks: (0u8..=255u8)
                .into_iter()
                .map(|i| {
                    Some((
                        true,
                        generate_chunk(&gen, region_x, region_y, i % 16, i / 16),
                    ))
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
/// Chunks stores 16x16 blocks
pub struct Chunk {
    pub active: bool,
    pub blocks: Vec<Block>,
}

/// Represents world meta data file with info on world
#[derive(Serialize, Deserialize)]
pub struct World {
    name: String,
    seed: u32,
    #[serde(skip_deserializing, skip_serializing, default)]    
    gen: Option<Generator>,
}
impl World {
    pub fn init(&mut self) {
        self.gen = Some(Generator::new(self.seed));
    }
    pub fn load(world_name: String) -> Result<Self> {
        let world_path = PathBuf::from("worlds").join(world_name);
        let reader = fs::File::open(world_path.join("world.json")).context("Failed to find world")?;
        serde_json::from_reader(reader).context("World file is broken")
    }
}

/// Converts region location to its file path
pub fn get_region_path(region_x: &i32, region_y: &i32) -> PathBuf {
    PathBuf::from(format!("world_file/regions/{region_x}.{region_y}.rf"))
}

/// Fetches region from game files and if not found returns none
pub fn get_region(region_x: i32, region_y: i32) -> Option<Region> {
    let path = get_region_path(&region_x, &region_y);
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) => return None,
    };
    match deserialize_from(file) {
        Ok(region) => Some(region),
        Err(_) => None,
    }
}

/// Save region to a file.
pub fn save_region(region_x: i32, region_y: i32, region: &Region) -> io::Result<()> {
    let path = get_region_path(&region_x, &region_y);

    // Serialize the region and save it to the file
    let file = fs::File::create(path)?;
    serialize_into(file, region).unwrap();

    Ok(())
}

pub fn generate_chunk(gen: &Generator, region_x: i32, region_y: i32, chunk_x: u8, chunk_y: u8) -> Chunk {
    let base_pos_x = ((region_x as i64) << 8) | (chunk_x as i64) << 4;
    let base_pos_y = ((region_y as i64) << 8) | (chunk_y as i64) << 4;

    let mut blocks = Vec::with_capacity(16 * 16 as usize);
    for i in 0..16 * 16 as usize {
        let world_x = base_pos_x | (15 - i % 16) as i64;
        let world_y = base_pos_y | (15 - i / 16) as i64;
        blocks.push(gen.get_block(world_x, world_y));
    }
    Chunk {
        active: false,
        blocks,
    }
}

pub fn world_to_chunk_pos(world_x: u64, world_y: u64) -> u8 {
    (((world_x >> 4) & 0b1111) << 4 | ((world_y >> 4) & 0b1111)) as u8
}
