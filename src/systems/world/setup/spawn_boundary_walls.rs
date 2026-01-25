use avian2d::prelude::*;
use bevy::prelude::*;

use crate::resources::terrain::{TerrainConfig, WORLD_MAX_CHUNK, WORLD_MIN_CHUNK};

/// Spawns four invisible static wall colliders at world boundaries
/// Prevents dynamic entities from escaping the playable area
pub fn spawn_boundary_walls(
    mut commands: Commands,
    terrain_config: Res<TerrainConfig>,
    mut spawned: Local<bool>,
) {
    // Only run once
    if *spawned {
        return;
    }

    // Calculate world bounds from terrain config
    // Formula: half_world = (chunks_per_side × chunk_size × tile_size) / 2
    let chunks_per_side = (WORLD_MAX_CHUNK - WORLD_MIN_CHUNK + 1) as f32;
    let half_world = (chunks_per_side * terrain_config.chunk_size as f32 * 16.0) / 2.0;

    // Wall thickness (100px provides large safety margin against tunneling)
    let wall_thickness = 100.0;
    let half_thickness = wall_thickness / 2.0;

    // Wall dimensions
    let horizontal_wall_width = half_world * 2.0;
    let horizontal_wall_height = wall_thickness;
    let vertical_wall_width = wall_thickness;
    let vertical_wall_height = half_world * 2.0;

    // Spawn four walls at boundaries
    // North wall (top)
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(horizontal_wall_width, horizontal_wall_height),
        Transform::from_xyz(0.0, half_world + half_thickness, 0.0),
        Name::new("Boundary Wall - North"),
    ));

    // South wall (bottom)
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(horizontal_wall_width, horizontal_wall_height),
        Transform::from_xyz(0.0, -(half_world + half_thickness), 0.0),
        Name::new("Boundary Wall - South"),
    ));

    // East wall (right)
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(vertical_wall_width, vertical_wall_height),
        Transform::from_xyz(half_world + half_thickness, 0.0, 0.0),
        Name::new("Boundary Wall - East"),
    ));

    // West wall (left)
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(vertical_wall_width, vertical_wall_height),
        Transform::from_xyz(-(half_world + half_thickness), 0.0, 0.0),
        Name::new("Boundary Wall - West"),
    ));

    *spawned = true;
    info!("Spawned 4 boundary walls at ±{} pixels", half_world);
}
