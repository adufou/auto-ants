use bevy::prelude::*;
use std::collections::HashMap;

/// Grid-based spatial partitioning for fast neighbor queries
/// Enables O(1) lookup of entities within a given radius
#[derive(Resource)]
pub struct SpatialGrid {
    /// Size of each grid cell in pixels
    pub cell_size: f32,
    /// Maps grid cell coordinates to list of entity IDs in that cell
    cells: HashMap<(i32, i32), Vec<Entity>>,
}

impl SpatialGrid {
    /// Create a new spatial grid with the specified cell size
    /// Recommended cell size: 2x the largest perception radius used
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::default(),
        }
    }

    /// Get grid cell coordinates for a world position
    fn world_to_cell(&self, pos: Vec2) -> (i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
        )
    }

    /// Clear all entities from the spatial grid
    /// Should be called at the start of each frame before repopulating
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    /// Insert an entity at a position
    pub fn insert(&mut self, entity: Entity, position: Vec2) {
        let cell = self.world_to_cell(position);
        self.cells.entry(cell).or_default().push(entity);
    }

    /// Query neighbors within radius of a position
    /// Returns all entities in nearby grid cells
    /// Note: May include entities outside the radius - caller should verify distance
    pub fn query_neighbors(&self, position: Vec2, radius: f32) -> Vec<Entity> {
        let center_cell = self.world_to_cell(position);
        let cell_radius = (radius / self.cell_size).ceil() as i32;

        let mut neighbors: Vec<Entity> = Vec::new();
        for dx in -cell_radius..=cell_radius {
            for dy in -cell_radius..=cell_radius {
                let cell = (center_cell.0 + dx, center_cell.1 + dy);
                if let Some(entities) = self.cells.get(&cell) {
                    neighbors.extend(entities.iter().copied());
                }
            }
        }
        neighbors
    }
}
