use bevy::prelude::*;

/// Loaded tilemap texture atlas and metadata
#[derive(Resource)]
pub struct TilemapAssets {
    /// Handle to the loaded texture (kitchen-sink.png)
    pub texture_handle: Handle<Image>,

    /// Texture index for grass tile (Kitchen Sink tileset index 176)
    pub grass_texture_index: u32,

    /// Texture index for water tile (Kitchen Sink tileset index 64)
    pub water_texture_index: u32,
}

impl TilemapAssets {
    pub fn new(texture_handle: Handle<Image>) -> Self {
        Self {
            texture_handle,
            // terrain.png: Assuming 16x16 grid (256÷16=16 tiles per side)
            // Will need to determine correct indices for grass and water
            // Starting with 0 and 1 as test values
            grass_texture_index: 0,
            water_texture_index: 1,
        }
    }
}
