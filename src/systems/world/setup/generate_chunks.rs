use crate::components::{GrassTile, WaterTile};
use crate::resources::{ChunkCoord, ChunkManager, TerrainConfig, TilemapAssets};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

/// Generate the entire 16x16 chunk world at startup
pub fn generate_world(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    config: Res<TerrainConfig>,
    tilemap_assets: Res<TilemapAssets>,
    mut generated: Local<bool>,
) {
    // Only run once
    if *generated {
        return;
    }

    info!("Starting world generation: 256 chunks...");

    // Get all chunk coordinates for the 16x16 world
    let all_chunks = chunk_manager.get_all_chunk_coords();

    // Generate all chunks
    for chunk_coord in all_chunks {
        if chunk_manager.loaded_chunks.contains(&chunk_coord) {
            continue; // Already loaded (shouldn't happen on first run)
        }

        // Spawn new chunk
        let chunk_entity = spawn_chunk(
            &mut commands,
            chunk_coord,
            &config,
            &tilemap_assets,
            &chunk_manager,
        );

        chunk_manager.loaded_chunks.insert(chunk_coord);
        chunk_manager
            .chunk_entities
            .insert(chunk_coord, chunk_entity);
    }

    *generated = true;
    info!("World generation complete: 256 chunks loaded");
}

fn spawn_chunk(
    commands: &mut Commands,
    chunk_coord: ChunkCoord,
    config: &TerrainConfig,
    assets: &TilemapAssets,
    chunk_manager: &ChunkManager,
) -> Entity {
    let map_size = TilemapSize {
        x: config.chunk_size,
        y: config.chunk_size,
    };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    // Spawn individual tiles
    for tile_x in 0..config.chunk_size {
        for tile_y in 0..config.chunk_size {
            // Calculate world tile coordinates
            let world_tile_x = chunk_coord.x * config.chunk_size as i32 + tile_x as i32;
            let world_tile_y = chunk_coord.y * config.chunk_size as i32 + tile_y as i32;

            // Sample Perlin noise to determine tile type
            let noise_value = chunk_manager.get_noise_at(
                world_tile_x as f64,
                world_tile_y as f64,
                config.noise_frequency,
            );
            let is_water = noise_value < config.water_threshold as f64;
            let texture_index = if is_water {
                assets.water_texture_index
            } else {
                assets.grass_texture_index
            };

            // Spawn tile entity
            let tile_pos = TilePos {
                x: tile_x,
                y: tile_y,
            };

            let mut tile_entity_commands = commands.spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: TileTextureIndex(texture_index),
                ..Default::default()
            });

            // Add marker component
            if is_water {
                tile_entity_commands.insert(WaterTile);
            } else {
                tile_entity_commands.insert(GrassTile);
            }

            let tile_entity = tile_entity_commands.id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    // Position tilemap so its local (0,0) tile is at the correct world location
    // Chunk offset in pixels: chunk_index * chunk_size * tile_size
    let chunk_world_x = (chunk_coord.x * config.chunk_size as i32) as f32 * 16.0;
    let chunk_world_y = (chunk_coord.y * config.chunk_size as i32) as f32 * 16.0;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TilemapGridSize { x: 16.0, y: 16.0 },
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(assets.texture_handle.clone()),
        tile_size: TilemapTileSize { x: 16.0, y: 16.0 },
        map_type: TilemapType::Square,
        anchor: TilemapAnchor::Center,
        transform: Transform::from_xyz(chunk_world_x, chunk_world_y, 0.0),
        ..Default::default()
    });

    tilemap_entity
}
