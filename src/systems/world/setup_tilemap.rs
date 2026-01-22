use crate::components::ChunkPosition;
use crate::resources::TilemapAssets;
use bevy::prelude::*;

pub fn setup_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load tilemap texture
    let texture_handle = asset_server.load("terrain/terrain.png");
    commands.insert_resource(TilemapAssets::new(texture_handle));

    // Spawn 2D camera with ChunkPosition tracking
    // Each tile is 16x16 pixels, chunks are 16x16 tiles = 256x256 pixels
    commands.spawn((Camera2d::default(), ChunkPosition::new(0, 0)));

    info!("Tilemap setup complete: camera spawned at origin");
}
