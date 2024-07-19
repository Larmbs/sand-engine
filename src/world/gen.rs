//! Generator functions for creating world from seed

use super::{blocks::Block, chunks::Chunk};
use noise::{self, NoiseFn};

/// Trait that defines requirements of a world generator
pub trait Generator {
    fn gen_block(&self, world_x: &i64, world_y: &i64) -> Block;
    fn gen_chunk(&self, region_x: &i32, region_y: &i32, chunk_x: &u8, chunk_y: &u8) -> Chunk {
        let base_pos_x = ((*region_x as i64) << 8) | (*chunk_x as i64) << 4;
        let base_pos_y = ((*region_y as i64) << 8) | (*chunk_y as i64) << 4;

        let mut blocks = Vec::with_capacity(16 * 16 as usize);
        for i in 0..16 * 16 as usize {
            let world_x = base_pos_x | (15 - i % 16) as i64;
            let world_y = base_pos_y | (15 - i / 16) as i64;
            blocks.push(self.gen_block(&world_x, &world_y));
        }
        Chunk::new(blocks.try_into().unwrap())
    }
}

pub struct WorldGenerator {
    gen: noise::Simplex,
}
impl Generator for WorldGenerator {
    fn gen_block(&self, world_x: &i64, world_y: &i64) -> Block {
        match self.get_height(world_x) {
            h if &h >= world_y => {
                let delta = h - world_y;

                match (world_y - self.noise1d(world_x, 3., 1.), delta) {
                    (y, d) if d < 4 && y < 54 => Block::Sand,
                    (y, d) if d < 9 && y < 54 => Block::SandStone,
                    (_, d) if d < 3 => Block::GrassBlock,
                    (_, d) if d < 8 => Block::Dirt,
                    (_, _) => Block::Stone,
                }
            }
            _ => match world_y {
                51i64..=i64::MAX => Block::Air,
                50 => Block::WaterEdge,
                i64::MIN..=49i64 => Block::Water,
            },
        }
    }
}
impl WorldGenerator {
    pub fn new(seed: u32) -> Self {
        WorldGenerator {
            gen: noise::Simplex::new(seed),
        }
    }
    fn get_height(&self, world_x: &i64) -> i64 {
        ((self.gen.get([*world_x as f64 / 256., 0.]) + 0.5) * 120. - 25.) as i64
            + self.noise1d(world_x, 1.5, 0.)
    }
    fn noise1d(&self, x: &i64, amplitude: f64, s: f64) -> i64 {
        (self.gen.get([s, *x as f64]) * amplitude) as i64
    }
}

pub struct FlatWorldGenerator {}
