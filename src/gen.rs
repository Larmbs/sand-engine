//! Generator functions for creating world from seed
use super::Block;
use noise::{self, NoiseFn};


#[allow(unused)]
pub struct Generator {
    seed: u32,
    gen: noise::Simplex,
}
impl Generator {
    pub fn new(seed: u32) -> Self {
        Generator {
            seed,
            gen: noise::Simplex::new(seed),
        }
    }
    pub fn get_block(&self, world_x: i64, world_y: i64) -> Block {
        if self.is_solid(world_x, world_y) {
            if self.is_solid(world_x, world_y + 5) {
                Block::STONE
            }
            else {
                if world_y > 50 {
                    Block::GRASS
                } else {
                    Block::SAND
                }
            }
        } else {
            if world_y > 48 {
                Block::AIR
            } else if world_y == 48  {
                Block::WaterEdge
            } else {
                Block::WATER
            }
        }
    }
    pub fn is_solid(&self, world_x: i64, world_y: i64) -> bool {
        self.gen.get([world_x as f64 / 128., world_y as f64 / 256.]) - world_y as f64 / 128. + 0.5 > 0.
    }
}
