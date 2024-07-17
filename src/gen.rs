//! Generator functions for creating world from seed
//!
use super::Block;
use noise::{self, NoiseFn};

/// World Generator which determines a blocks type
pub struct Generator {
    gen: noise::Simplex,
}
impl Generator {
    pub fn new(seed: u32) -> Self {
        Generator {
            gen: noise::Simplex::new(seed),
        }
    }
    pub fn get_block(&self, world_x: i64, world_y: i64) -> Block {
        match self.get_height(world_x) {
            h if h >= world_y => {
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
    pub fn get_height(&self, world_x: i64) -> i64 {
        ((self.gen.get([world_x as f64 / 256., 0.]) + 0.5) * 200. - 25.) as i64
            + self.noise1d(world_x, 1.5, 0.)
    }
    fn noise1d(&self, x: i64, amplitude: f64, s: f64) -> i64 {
        (self.gen.get([s, x as f64]) * amplitude) as i64
    }
}
