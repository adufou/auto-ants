use crate::components::ChunkPosition;
use crate::resources::{ChunkManager, TerrainConfig};
use bevy::prelude::*;
use super::helpers::get_camera_chunk;

pub fn despawn_chunks(
    mut commands: Commands,
    camera_query: Query<&ChunkPosition, With<Camera2d>>,
    mut chunk_manager: ResMut<ChunkManager>,
    config: Res<TerrainConfig>,
) {
    let camera_chunk = match get_camera_chunk(&camera_query) {
        Some(chunk) => chunk,
        None => return,
    };

    let visible_chunks = chunk_manager.calculate_visible_chunks(
        camera_chunk.x,
        camera_chunk.y,
        config.chunk_view_distance,
    );

    // Find chunks to despawn
    let chunks_to_despawn: Vec<_> = chunk_manager
        .loaded_chunks
        .iter()
        .filter(|coord| !visible_chunks.contains(coord))
        .copied()
        .collect();

    // Despawn and cleanup
    for chunk_coord in chunks_to_despawn {
        if let Some(entity) = chunk_manager.chunk_entities.remove(&chunk_coord) {
            // Use despawn and rely on bevy_ecs_tilemap to clean up child tiles
            commands.entity(entity).despawn();
            chunk_manager.loaded_chunks.remove(&chunk_coord);
            info!("Despawned chunk ({}, {})", chunk_coord.x, chunk_coord.y);
        }
    }
}
