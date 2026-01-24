use crate::resources::{ChunkManager, TerrainConfig};
use crate::systems::world::{
    camera_controls, generate_world, performance_monitor, random_walk_movement,
    setup_entities, setup_tilemap, spawn_human, terrain_tuning, track_camera_chunk,
};
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        // Insert resources
        app.insert_resource(TerrainConfig::default());
        app.insert_resource(ChunkManager::new(12345));

        // Startup systems (generate_world must run after setup_tilemap to access TilemapAssets)
        app.add_systems(Startup, (setup_tilemap, setup_entities).chain());
        app.add_systems(Startup, generate_world.after(setup_tilemap));

        // Update systems
        app.add_systems(
            Update,
            (
                terrain_tuning,
                performance_monitor,
                spawn_human,
                random_walk_movement,
            ),
        ); // Independent systems

        // Camera controls chain
        app.add_systems(
            Update,
            (
                camera_controls,    // Move camera with WASD
                track_camera_chunk, // Update camera's chunk position
            )
                .chain(),
        );
    }
}
