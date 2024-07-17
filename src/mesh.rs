//! Module which defines meshing for the sand_engine
use super::{Block, Chunk};
use macroquad::prelude::Rect;

/// Distinguishes culled meshes from greedy one
pub enum MeshType {
    CULLED,
    GREEDY,
}

/// Represents rectangles to be drawn within a chunk
pub struct ChunkMesh {
    pub mesh_type: MeshType,
    pub mesh: Vec<(Block, Rect)>,
}
impl ChunkMesh {
    /// Creates a chunk mesh but uses the greedy algorithm to solve it
    pub fn greedy_mesh(chunk: &Chunk) -> ChunkMesh {
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
                        y: if block_type == Block::WaterEdge {y as f32 + 0.2} else {y as f32},
                        w: w as f32,
                        h:if block_type == Block::WaterEdge {h as f32 - 0.2} else {h as f32},
                    };
                    
                    mesh.push((block_type, rect));
                    

                    y += h;
                }
            }
        }
        ChunkMesh {
            mesh_type: MeshType::GREEDY,
            mesh,
        }
    }
    /// Creates a chunk mesh but uses a culled algorithm to solve
    pub fn culled_mesh(chunk: &Chunk) -> ChunkMesh {
        let mesh = chunk
            .blocks
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, block)| {
                (
                    block,
                    Rect {
                        x: (i % 16) as f32,
                        y: (i / 16) as f32,
                        w: 1.,
                        h: 1.,
                    },
                )
            })
            .collect();
        ChunkMesh {
            mesh_type: MeshType::CULLED,
            mesh,
        }
    }
}
