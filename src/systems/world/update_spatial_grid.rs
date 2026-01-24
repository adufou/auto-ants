use crate::components::Human;
use crate::resources::SpatialGrid;
use bevy::prelude::*;

/// System that rebuilds the spatial grid each frame with current human positions
/// Must run before any systems that query the spatial grid for neighbors
pub fn update_spatial_grid(
    mut spatial_grid: ResMut<SpatialGrid>,
    humans: Query<(Entity, &Transform), With<Human>>,
) {
    spatial_grid.clear();

    for (entity, transform) in &humans {
        let pos = Vec2::new(transform.translation.x, transform.translation.y);
        spatial_grid.insert(entity, pos);
    }
}
