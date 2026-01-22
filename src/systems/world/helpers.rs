use bevy::prelude::*;
use crate::components::ChunkPosition;

/// Helper to safely get camera chunk position
/// Returns None if camera doesn't exist or multiple cameras found
pub fn get_camera_chunk(camera_query: &Query<&ChunkPosition, With<Camera2d>>) -> Option<ChunkPosition> {
    if camera_query.is_empty() {
        return None;
    }

    camera_query.single().ok().copied()
}
