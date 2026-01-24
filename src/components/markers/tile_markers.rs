use bevy::prelude::*;

/// Marker component for grass tiles
/// Attach to tilemap tile entities to identify grass terrain
#[derive(Component, Debug, Clone, Copy)]
pub struct GrassTile;

/// Marker component for water tiles
/// Attach to tilemap tile entities to identify water terrain
#[derive(Component, Debug, Clone, Copy)]
pub struct WaterTile;
