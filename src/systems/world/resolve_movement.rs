use crate::components::{
    CohesionInfluence, CurrentDirection, Human, MovementVelocity, RandomWalkInfluence,
};
use crate::resources::{MovementConfig, SpatialGrid};
use bevy::prelude::*;
use std::f32::consts::PI;

/// System that combines all movement influences into a final velocity
/// Uses percentage-based weight normalization
/// Gradually rotates current direction toward desired direction with max rotation rate
pub fn resolve_movement(
    mut query: Query<
        (
            Entity,
            &Transform,
            &mut MovementVelocity,
            &mut CurrentDirection,
            &RandomWalkInfluence,
            Option<&CohesionInfluence>,
        ),
        With<Human>,
    >,
    spatial_grid: Res<SpatialGrid>,
    positions: Query<&Transform, With<Human>>,
    movement_config: Res<MovementConfig>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for (entity, transform, mut velocity, mut current_dir, random_walk, cohesion_opt) in
        &mut query
    {
        // Step 1: Calculate DESIRED direction from all influences
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

        let desired_direction = if total_weight > 0.0 {
            (combined_direction / total_weight).normalize_or_zero()
        } else {
            Vec2::ZERO
        };

        // Step 2: Rotate current direction toward desired direction with max rate
        if desired_direction.length_squared() > 0.001 {
            let current_angle = current_dir.direction.y.atan2(current_dir.direction.x);
            let desired_angle = desired_direction.y.atan2(desired_direction.x);

            // Calculate shortest angle difference (-PI to PI)
            let mut angle_diff = desired_angle - current_angle;
            while angle_diff > PI {
                angle_diff -= 2.0 * PI;
            }
            while angle_diff < -PI {
                angle_diff += 2.0 * PI;
            }

            // Clamp rotation to max rate
            let max_rotation_this_frame = movement_config.max_rotation_rate * dt;
            let rotation = angle_diff.clamp(-max_rotation_this_frame, max_rotation_this_frame);

            let new_angle = current_angle + rotation;
            current_dir.direction = Vec2::new(new_angle.cos(), new_angle.sin());
        }

        // Step 3: Set velocity using CURRENT direction (not desired)
        velocity.velocity = current_dir.direction * movement_config.base_speed;
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
