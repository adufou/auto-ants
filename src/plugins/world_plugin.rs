use crate::resources::{ChunkManager, TerrainConfig};
use crate::systems::world::{
    camera_controls, despawn_chunks, generate_chunks, performance_monitor, random_walk_movement,
    setup_entities, setup_tilemap, spawn_ant, terrain_tuning, track_camera_chunk,
};
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        // Insert resources
        app.insert_resource(TerrainConfig::default());
        app.insert_resource(ChunkManager::new(12345));

        // Startup systems
        app.add_systems(Startup, (setup_tilemap, setup_entities));

        // Update systems
        app.add_systems(
            Update,
            (terrain_tuning, performance_monitor, spawn_ant, random_walk_movement),
        ); // Independent systems

        // Ordered chain for chunk generation/loading
        app.add_systems(
            Update,
            (
                camera_controls,    // 0. Move camera with WASD
                track_camera_chunk, // 1. Update camera's chunk position
                generate_chunks,    // 2. Spawn new chunks in view
                despawn_chunks,     // 3. Remove chunks out of view
            )
                .chain(),
        );
    }
}
