use crate::components::{CohesionInfluence, Human, MovementVelocity, RandomWalkInfluence};
use crate::resources::{MovementConfig, SpatialGrid};
use bevy::prelude::*;

/// System that combines all movement influences into a final velocity
/// Uses percentage-based weight normalization
pub fn resolve_movement(
    mut query: Query<
        (
            Entity,
            &Transform,
            &mut MovementVelocity,
            &RandomWalkInfluence,
            Option<&CohesionInfluence>,
        ),
        With<Human>,
    >,
    spatial_grid: Res<SpatialGrid>,
    positions: Query<&Transform, With<Human>>,
    movement_config: Res<MovementConfig>,
) {
    for (entity, transform, mut velocity, random_walk, cohesion_opt) in &mut query {
        // Collect weighted directions
        let mut combined_direction = Vec2::ZERO;
        let mut total_weight = 0.0;

        // Random walk contribution
        combined_direction += random_walk.direction.normalize_or_zero() * random_walk.weight;
        total_weight += random_walk.weight;

        // Cohesion contribution
        if let Some(cohesion) = cohesion_opt {
            let steering = calculate_cohesion_steering(
                entity,
                transform,
                cohesion,
                &spatial_grid,
                &positions,
            );
            combined_direction += steering.normalize_or_zero() * cohesion.weight;
            total_weight += cohesion.weight;
        }

        // Normalize by total weight and apply base speed
        if total_weight > 0.0 {
            velocity.velocity = (combined_direction / total_weight).normalize_or_zero()
                * movement_config.base_speed;
        } else {
            velocity.velocity = Vec2::ZERO;
        }
    }
}

/// Calculate the steering force for cohesion behavior
/// Returns a force vector pointing toward the center of mass of nearby entities
fn calculate_cohesion_steering(
    entity: Entity,
    transform: &Transform,
    cohesion: &CohesionInfluence,
    spatial_grid: &SpatialGrid,
    positions: &Query<&Transform, With<Human>>,
) -> Vec2 {
    let pos = Vec2::new(transform.translation.x, transform.translation.y);
    let neighbors = spatial_grid.query_neighbors(pos, cohesion.perception_radius);

    let mut center_of_mass = Vec2::ZERO;
    let mut count = 0;

    for neighbor_entity in neighbors {
        if neighbor_entity == entity {
            continue; // Skip self
        }

        if let Ok(neighbor_transform) = positions.get(neighbor_entity) {
            let neighbor_pos = Vec2::new(
                neighbor_transform.translation.x,
                neighbor_transform.translation.y,
            );
            let distance = pos.distance(neighbor_pos);

            if distance < cohesion.perception_radius && distance > 0.01 {
                center_of_mass += neighbor_pos;
                count += 1;
            }
        }
    }

    if count > 0 {
        center_of_mass /= count as f32;
        // Return direction only, no max_force multiplication
        (center_of_mass - pos).normalize_or_zero()
    } else {
        Vec2::ZERO
    }
}
