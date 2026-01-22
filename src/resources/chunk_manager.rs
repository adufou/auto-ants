use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use std::collections::{HashMap, HashSet};

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

    /// Calculate which chunks should be visible based on camera position
    pub fn calculate_visible_chunks(
        &self,
        camera_chunk_x: i32,
        camera_chunk_y: i32,
        view_distance: i32,
    ) -> HashSet<ChunkCoord> {
        let mut visible = HashSet::new();

        for dx in -view_distance..=view_distance {
            for dy in -view_distance..=view_distance {
                visible.insert(ChunkCoord::new(camera_chunk_x + dx, camera_chunk_y + dy));
            }
        }

        visible
    }

    /// Get noise value for world tile position
    pub fn get_noise_at(&self, world_x: f64, world_y: f64, frequency: f64) -> f64 {
        self.noise_generator
            .get([world_x * frequency, world_y * frequency])
    }
}
