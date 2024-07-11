use super::{Block, Chunk};
use macroquad::prelude::Rect;

pub struct ChunkMesh {
    pub mesh: Vec<(Block, Rect)>,
}

impl ChunkMesh {
    pub fn from_chunk(chunk: &Chunk) -> ChunkMesh {
        let mut blocks = chunk.blocks.clone();
        let mut mesh: Vec<(Block, Rect)> = Vec::new();

        for x in 0..16 {
            let mut y = 0;
            while y < 16 {
                if blocks[x + y * 16] == Block::AIR {
                    y += 1;
                    continue;
                } else {
                    let block_type = blocks[x + y * 16].clone();
                    let mut w = 1;
                    let mut h = 1;

                    // Determine height
                    while y + h < 16 && blocks[x + (y + h) * 16] == block_type {
                        h += 1;
                    }

                    // Determine width
                    'w: for wx in (x + 1)..16 {
                        for wy in y..(y + h) {
                            if blocks[wx + wy * 16] != block_type {
                                break 'w;
                            }
                        }
                        w += 1;
                    }

                    // Mark visited
                    for dx in 0..w {
                        for dy in 0..h {
                            blocks[(x + dx) + (y + dy) * 16] = Block::AIR;
                        }
                    }

                    // Create the rectangle
                    let rect = Rect {
                        x: x as f32,
                        y: y as f32,
                        w: w as f32,
                        h: h as f32,
                    };
                    mesh.push((block_type, rect));

                    y += h;
                }
            }
        }

        ChunkMesh { mesh }
    }
}
