use crate::resources::EntityAssets;
use bevy::prelude::*;

/// Startup system that loads entity sprite assets
/// Loads entities.png and creates a texture atlas layout for 16x16 grid
pub fn setup_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load entity sprite texture
    let texture_handle = asset_server.load("entities/entities.png");

    // Create texture atlas layout for 16x16 pixel tiles
    // Assuming entities.png is same size as terrain.png (256x256 = 16x16 grid)
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(16, 16), // tile size
        16,                 // columns
        16,                 // rows
        None,               // no padding
        None,               // no offset
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Store handles in resource
    commands.insert_resource(EntityAssets::new(texture_handle, texture_atlas_layout));

    info!("Entity assets loaded: entities.png with 16x16 grid layout");
}
