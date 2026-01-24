use crate::components::{Human, HumanRelationships};
use crate::resources::SpatialGrid;
use crate::systems::world::social::relationship_utils::apply_tanh_modifier;
use bevy::prelude::*;
use rand::Rng;

/// Distance threshold for relationship updates (in pixels)
const RELATIONSHIP_RADIUS: f32 = 64.0;

/// Maximum random change per second (before modifier)
const BASE_CHANGE_AMOUNT: f32 = 0.05;

/// System that updates relationships between nearby humans
/// Runs every frame for humans within 32px of each other
pub fn update_close_humans_relationships(
    mut humans: Query<(Entity, &Transform, &mut HumanRelationships), With<Human>>,
    spatial_grid: Res<SpatialGrid>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let mut rng = rand::thread_rng();

    // Collect updates to apply after iteration (avoid double-borrow)
    let mut updates: Vec<(Entity, Entity, f32)> = Vec::new();

    for (entity, transform, relationships) in &humans {
        let pos = Vec2::new(transform.translation.x, transform.translation.y);
        let neighbors = spatial_grid.query_neighbors(pos, RELATIONSHIP_RADIUS);

        for neighbor_entity in neighbors {
            // Skip self
            if neighbor_entity == entity {
                continue;
            }

            // Verify distance is actually within radius (spatial grid returns approximation)
            if let Ok((_, neighbor_transform, _)) = humans.get(neighbor_entity) {
                let neighbor_pos = Vec2::new(
                    neighbor_transform.translation.x,
                    neighbor_transform.translation.y,
                );
                let distance = pos.distance(neighbor_pos);

                if distance <= RELATIONSHIP_RADIUS && distance > 0.01 {
                    // Get current relationship
                    let current_relationship = relationships.get_relationship(neighbor_entity);

                    // Generate random base change
                    let base_change = rng.gen_range(-BASE_CHANGE_AMOUNT..=BASE_CHANGE_AMOUNT);

                    // Apply tanh modifier
                    let modified_change = apply_tanh_modifier(current_relationship, base_change);

                    // Calculate new relationship (scaled by delta time for frame-rate independence)
                    let new_relationship = current_relationship + (modified_change * dt);

                    // Queue update
                    updates.push((entity, neighbor_entity, new_relationship));
                }
            }
        }
    }

    // Apply all updates
    for (entity, neighbor_entity, new_relationship) in updates {
        if let Ok((_, _, mut relationships)) = humans.get_mut(entity) {
            relationships.update_relationship(neighbor_entity, new_relationship);
        }
    }
}
