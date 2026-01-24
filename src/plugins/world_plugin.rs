use crate::resources::{
    CameraConfig, ChunkManager, DebugConfig, MovementConfig, SpatialGrid, TerrainConfig,
};
use crate::systems::world::{
    apply_movement, calculate_random_walk, camera_controls, camera_zoom, generate_world,
    performance_monitor, resolve_movement, setup_entities, setup_tilemap, spawn_human,
    terrain_tuning, track_camera_chunk, update_spatial_grid, vision_debug_gizmos,
};
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        // Insert resources
        app.insert_resource(CameraConfig::default());
        app.insert_resource(DebugConfig::default());
        app.insert_resource(TerrainConfig::default());
        app.insert_resource(ChunkManager::new(12345));
        app.insert_resource(MovementConfig::default());
        app.insert_resource(SpatialGrid::new(300.0));

        // Startup systems (generate_world must run after setup_tilemap to access TilemapAssets)
        app.add_systems(Startup, (setup_tilemap, setup_entities).chain());
        app.add_systems(Startup, generate_world.after(setup_tilemap));

        // Update systems
        app.add_systems(
            Update,
            (
                terrain_tuning,
                performance_monitor,
                vision_debug_gizmos,
                spawn_human,
                // New movement pipeline (replaces random_walk_movement)
                (
                    update_spatial_grid,
                    calculate_random_walk,
                    (resolve_movement, apply_movement).chain(),
                )
                    .chain(),
            ),
        ); // Independent systems

        // Camera controls chain
        app.add_systems(
            Update,
            (
                camera_controls,    // Move camera with WASD
                camera_zoom,        // Zoom camera with mouse wheel
                track_camera_chunk, // Update camera's chunk position
            )
                .chain(),
        );
    }
}
