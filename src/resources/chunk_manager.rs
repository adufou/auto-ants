use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use std::collections::{HashMap, HashSet};

use crate::resources::terrain_config::{WORLD_MAX_CHUNK, WORLD_MIN_CHUNK};

/// Chunk coordinate in world space
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

impl ChunkCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Manages loaded chunks and noise generation
#[derive(Resource)]
pub struct ChunkManager {
    /// Currently loaded chunk coordinates
    pub loaded_chunks: HashSet<ChunkCoord>,

    /// Chunk coordinate -> tilemap entity ID
    /// Needed for despawning chunks
    pub chunk_entities: HashMap<ChunkCoord, Entity>,

    /// Perlin noise generator (seeded for reproducibility)
    pub noise_generator: Perlin,
}

impl ChunkManager {
    pub fn new(seed: u32) -> Self {
        Self {
            loaded_chunks: HashSet::new(),
            chunk_entities: HashMap::new(),
            noise_generator: Perlin::new(seed),
        }
    }

    /// Get all chunk coordinates for the fixed 16x16 world
    pub fn get_all_chunk_coords(&self) -> Vec<ChunkCoord> {
        let mut coords = Vec::with_capacity(256);
        for x in WORLD_MIN_CHUNK..=WORLD_MAX_CHUNK {
            for y in WORLD_MIN_CHUNK..=WORLD_MAX_CHUNK {
                coords.push(ChunkCoord::new(x, y));
            }
        }
        coords
    }

    /// Get noise value for world tile position
    pub fn get_noise_at(&self, world_x: f64, world_y: f64, frequency: f64) -> f64 {
        self.noise_generator
            .get([world_x * frequency, world_y * frequency])
    }
}
