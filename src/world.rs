use bincode::{serialize_into, deserialize_from};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use std::io;
use macroquad::color::Color;
use noise::{self, NoiseFn};

/// There are only 128 blocks
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Block(pub u8);
impl Block {
    /// Returns the color of the block, None means it is transparent
    pub fn get_color(&self) -> Color {
        match self.0 {
            0 => Color::new(0., 0., 0., 0.), // Air
            1 => Color::from_hex(0x307a2a), // Grass
            2 => Color::from_hex(0xba7938), // Dirt
            3 => Color::from_hex(0x515357), // Stone
            4 => Color::from_hex(0x4b53eb), // Water
            5 => Color::from_hex(0xbbc26d), // Sand
            _ => Color::from_hex(0x7e2e87), // Unknown
        }
    }
}

/// Stores 16x16 chunks
#[derive(Serialize, Deserialize)]
pub struct Region {
    pub modified: bool,
    // Cannot enforce Vec size because serde is weird, but this will hold 16x16 blocks
    // Each entry carries bool determining if it is new
    pub chunks: Vec<Option<(bool, Chunk)>>,
}
impl Region {
    pub fn new(region_x: i32, region_y: i32, seed: u32) -> Self {
        Region {
            modified: true,
            chunks: (0u8..=255u8).into_iter().map(|i| Some((true, generate_chunk(seed, region_x, region_y, i % 16, i / 16)))).collect()
        }
    }
}

#[derive(Serialize, Deserialize)]
/// Chunks stores 16x16 blocks
pub struct Chunk {
    pub blocks: Vec<Block>,
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

pub fn generate_chunk(seed: u32, region_x: i32, region_y: i32, chunk_x: u8, chunk_y: u8) -> Chunk {
    let gen = noise::Simplex::new(seed);

    let base_pos_x = ((region_x as i64) << 8) | (chunk_x as i64) << 4;
    let base_pos_y = ((region_y as i64) << 8) | (chunk_y as i64) << 4;

    let mut blocks = Vec::with_capacity(16*16 as usize);
    for i in 0..16*16 as usize {
        let world_x = base_pos_x | (15 - i % 16) as i64;
        let world_y = base_pos_y | (15 - i / 16) as i64;

        let block = Block {0: ((gen.get([world_x as f64 / 128., world_y as f64 / 128.]) * 4. - world_y as f64 / 16. + 8.) as u8)};
        blocks.push(block)
    }
    Chunk{blocks}
}

pub fn world_to_chunk_pos(world_x: u64, world_y: u64) -> u8 {
    (((world_x >> 4) & 0b1111) << 4 | ((world_y >> 4) & 0b1111)) as u8
}