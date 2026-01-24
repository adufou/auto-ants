use crate::resources::ChunkManager;
use bevy::prelude::*;

/// Monitor and log performance metrics
pub fn performance_monitor(chunk_manager: Res<ChunkManager>, mut timer: Local<u32>) {
    *timer += 1;

    // Log performance periodically (every 120 frames ~2 seconds at 60 FPS)
    if *timer % 120 == 0 {
        let total_tiles = chunk_manager.loaded_chunks.len() * 256;
        info!(
            "Performance - Chunks: {}, Total Tiles: {}",
            chunk_manager.loaded_chunks.len(),
            total_tiles
        );
    }
}
