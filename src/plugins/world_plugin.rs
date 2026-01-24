use crate::resources::{
    CameraConfig, ChunkManager, DebugConfig, MovementConfig, PhysicsConfig, SpatialGrid,
    TerrainConfig,
};
use crate::systems::camera::{
    camera_controls, camera_follow_human, camera_zoom, track_camera_chunk,
};
use crate::systems::debug::{performance_monitor, terrain_tuning, vision_debug_gizmos};
use crate::systems::world::{
    apply_movement, calculate_random_walk, generate_world, resolve_movement, setup_entities,
    setup_tilemap, spawn_human, update_close_humans_relationships, update_spatial_grid,
};
use avian2d::prelude::*;
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
        app.insert_resource(PhysicsConfig::default());
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
            ),
        );

        // Movement pipeline - runs before Avian physics
        app.add_systems(
            Update,
            (
                update_spatial_grid,
                update_close_humans_relationships, // Update relationships after spatial grid
                calculate_random_walk,
                (resolve_movement, apply_movement).chain(),
            )
                .chain()
                .before(PhysicsSystems::StepSimulation),
        );

        // Camera controls chain
        app.add_systems(
            Update,
            (
                camera_follow_human, // Follow selected human (runs first)
                camera_controls,     // Move camera with WASD (disabled during follow)
                camera_zoom,         // Zoom camera with mouse wheel
                track_camera_chunk,  // Update camera's chunk position
            )
                .chain(),
        );
    }
}
