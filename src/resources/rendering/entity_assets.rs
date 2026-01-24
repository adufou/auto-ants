use bevy::prelude::*;

/// Resource that holds handles to entity sprite assets
/// Manages the entities.png sprite sheet and its texture atlas layout
#[derive(Resource)]
pub struct EntityAssets {
    pub texture_handle: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}

impl EntityAssets {
    pub fn new(texture_handle: Handle<Image>, texture_atlas_layout: Handle<TextureAtlasLayout>) -> Self {
        Self {
            texture_handle,
            texture_atlas_layout,
        }
    }
}
