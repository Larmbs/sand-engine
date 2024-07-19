//! Defines the structure of a chunk

/* Serialization Crates */
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::blocks::Block;
use chrono::{DateTime, Local};

/// Structure that represents a World Chunk
#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chunk {
    #[serde_as(as = "[_; 16*16]")]
    pub blocks: [Block; 16 * 16],
    #[serde(skip_serializing, default = "Local::now", skip_deserializing)]
    pub last_used: DateTime<Local>,
}
impl Chunk {
    /// Creates a new chunk given an array of Blocks
    pub fn new(blocks: [Block; 16 * 16]) -> Self {
        Self {
            blocks,
            last_used: Local::now(),
        }
    }
}
