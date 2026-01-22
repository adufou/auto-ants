use crate::components::ChunkPosition;
use crate::resources::ChunkManager;
use bevy::prelude::*;

pub fn despawn_chunks(
    mut commands: Commands,
    camera_query: Query<&ChunkPosition, With<Camera2d>>,
    mut chunk_manager: ResMut<ChunkManager>,
    config: Res<crate::resources::TerrainConfig>,
) {
    if camera_query.is_empty() {
        return;
    }

    let camera_chunk = match camera_query.single() {
        Ok(chunk) => chunk,
        Err(_) => return,
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
