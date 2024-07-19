//! Module which defines blocks and their respective colors
use macroquad::prelude::{Color, PURPLE};
use serde::{Deserialize, Serialize};

/// Block types within sand engine
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[repr(u8)] // Ensures the struct is of size u8
pub enum Block {
    Air,
    GrassBlock,
    Dirt,
    Stone,
    Water,
    WaterEdge,
    Sand,
    SandStone,
    OakWood,
    OakLeave,
    Fire,
}
impl Block {
    /// Returns the blocks respective color
    pub fn color(&self) -> Color {
        match self {
            Block::Air => Color::new(0., 0., 0., 0.),
            Block::GrassBlock => Color::from_hex(0x307a2a),
            Block::Dirt => Color::from_hex(0xba7938),
            Block::Stone => Color::from_hex(0x515357),
            Block::Water | Block::WaterEdge => Color::from_hex(0x4b53eb),
            Block::Sand => Color::from_hex(0xbbc26d),
            Block::SandStone => Color::from_hex(0xe1e897),
            _ => PURPLE,
        }
    }
    /// Determines if a block should be treated as transparent or not for collisions
    pub fn is_solid(&self) -> bool {
        match self {
            Block::Water | Block::Air | Block::WaterEdge => false,
            _ => true,
        }
    }
}
