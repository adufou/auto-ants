use bevy::prelude::*;

/// Configuration for procedural terrain generation
#[derive(Resource, Debug, Clone)]
pub struct TerrainConfig {
    /// Water appears when noise value < this threshold
    /// Range: -1.0 to 1.0 (Perlin noise output range)
    /// Default 0.0 = ~50% water, 0.2 = ~40% water, -0.2 = ~60% water
    pub water_threshold: f32,

    /// Seed for noise generator (reproducible worlds)
    pub noise_seed: u32,

    /// Noise frequency/scale (higher = more varied terrain)
    /// Start with 0.05 for gentle rolling terrain
    pub noise_frequency: f64,

    /// Chunk size in tiles (16x16 per spec)
    pub chunk_size: u32,

    /// Chunk view distance (8 chunks = 16x16 total grid)
    pub chunk_view_distance: i32,
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            water_threshold: 0.0,
            noise_seed: 12345,
            noise_frequency: 0.05,
            chunk_size: 16,
            chunk_view_distance: 8,
        }
    }
}
