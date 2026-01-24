use bevy::prelude::*;

/// World size configuration (16x16 chunk world)
pub const WORLD_MIN_CHUNK: i32 = -8; // Minimum chunk coordinate
pub const WORLD_MAX_CHUNK: i32 = 7; // Maximum chunk coordinate

/// Configuration for procedural terrain generation
#[derive(Resource, Debug, Clone)]
pub struct TerrainConfig {
    /// Water appears when noise value < this threshold
    /// Range: -1.0 to 1.0 (Perlin noise output range)
    /// Default 0.0 = ~50% water, 0.2 = ~40% water, -0.2 = ~60% water
    pub water_threshold: f32,

    /// Noise frequency/scale (higher = more varied terrain)
    /// Start with 0.05 for gentle rolling terrain
    pub noise_frequency: f64,

    /// Chunk size in tiles (16x16 per spec)
    pub chunk_size: u32,
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            water_threshold: 0.0,
            noise_frequency: 0.05,
            chunk_size: 16,
        }
    }
}
