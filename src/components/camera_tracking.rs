use bevy::prelude::*;

/// Tracks which chunk the camera is currently in
/// Updated each frame to drive chunk loading/unloading
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
}

impl ChunkPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_world_position(world_x: f32, world_y: f32, chunk_size: u32) -> Self {
        let tile_size = 16.0; // Matches TilemapGridSize
        let tiles_per_chunk = chunk_size as f32;
        let chunk_world_size = tiles_per_chunk * tile_size;

        Self {
            x: (world_x / chunk_world_size).floor() as i32,
            y: (world_y / chunk_world_size).floor() as i32,
        }
    }
}
