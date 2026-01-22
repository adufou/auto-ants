use crate::components::ChunkPosition;
use crate::resources::TerrainConfig;
use bevy::prelude::*;

pub fn track_camera_chunk(
    mut camera_query: Query<(&Transform, &mut ChunkPosition), With<Camera2d>>,
    config: Res<TerrainConfig>,
) {
    for (transform, mut chunk_pos) in &mut camera_query {
        let new_chunk = ChunkPosition::from_world_position(
            transform.translation.x,
            transform.translation.y,
            config.chunk_size,
        );

        if *chunk_pos != new_chunk {
            info!("Camera moved to chunk ({}, {})", new_chunk.x, new_chunk.y);
            *chunk_pos = new_chunk;
        }
    }
}
