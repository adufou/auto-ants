use crate::components::{
    CenterPullInfluence, CohesionInfluence, CurrentDirection, DesiredDirection, Human,
    HumanRelationships, MovementVelocity, RandomWalkInfluence, RelationshipInfluence,
};
use crate::resources::{MovementConfig, SpatialGrid};
use bevy::prelude::*;
use std::f32::consts::PI;

/// World radius: (16 chunks × 16 tiles × 16 pixels) / 2 = 2048
const WORLD_HALF_SIZE: f32 = 2048.0;

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
            &mut DesiredDirection,
            &RandomWalkInfluence,
            Option<&CohesionInfluence>,
            Option<&RelationshipInfluence>,
            Option<&CenterPullInfluence>,
            &HumanRelationships,
        ),
        With<Human>,
    >,
    spatial_grid: Res<SpatialGrid>,
    positions: Query<&Transform, With<Human>>,
    movement_config: Res<MovementConfig>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for (
        entity,
        transform,
        mut velocity,
        mut current_dir,
        mut desired_dir,
        random_walk,
        cohesion_opt,
        relationship_opt,
        center_pull_opt,
        relationships,
    ) in &mut query
    {
        // Step 1: Calculate DESIRED direction from all influences
        let mut combined_direction = Vec2::ZERO;
        let mut total_weight = 0.0;

        // Random walk contribution
        combined_direction += random_walk.direction.normalize_or_zero() * random_walk.weight;
        total_weight += random_walk.weight;

        // Cohesion contribution
        if let Some(cohesion) = cohesion_opt {
            let steering =
                calculate_cohesion_steering(entity, transform, cohesion, &spatial_grid, &positions);
            combined_direction += steering.normalize_or_zero() * cohesion.weight;
            total_weight += cohesion.weight;
        }

        // Relationship influence contribution
        if let Some(relationship_influence) = relationship_opt {
            let steering = calculate_relationship_steering(
                entity,
                transform,
                relationship_influence,
                relationships,
                &spatial_grid,
                &positions,
            );
            combined_direction += steering * relationship_influence.weight;
            total_weight += relationship_influence.weight;
        }

        // Center pull contribution
        if let Some(center_pull) = center_pull_opt {
            let steering = calculate_center_pull_steering(transform);
            combined_direction += steering * center_pull.weight;
            total_weight += center_pull.weight;
        }

        let desired_direction = if total_weight > 0.0 {
            (combined_direction / total_weight).normalize_or_zero()
        } else {
            Vec2::ZERO
        };

        // Store desired direction for visualization
        desired_dir.direction = desired_direction;

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

/// Calculate the steering force based on relationships with nearby entities
/// Returns a normalized direction vector pointing:
/// - TOWARD allies (positive relationships)
/// - AWAY FROM enemies (negative relationships)
fn calculate_relationship_steering(
    entity: Entity,
    transform: &Transform,
    relationship_influence: &RelationshipInfluence,
    relationships: &HumanRelationships,
    spatial_grid: &SpatialGrid,
    positions: &Query<&Transform, With<Human>>,
) -> Vec2 {
    let pos = Vec2::new(transform.translation.x, transform.translation.y);
    let neighbors = spatial_grid.query_neighbors(pos, relationship_influence.perception_radius);

    let mut steering = Vec2::ZERO;

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

            if distance < relationship_influence.perception_radius && distance > 0.01 {
                let relationship_score = relationships.get_relationship(neighbor_entity);

                // Skip if no relationship (neutral/unknown)
                if relationship_score.abs() < 0.01 {
                    continue;
                }

                let direction = (neighbor_pos - pos).normalize_or_zero();

                if relationship_score > 0.0 {
                    // Positive = ally = move TOWARD
                    steering +=
                        direction * relationship_score * relationship_influence.attraction_strength;
                } else {
                    // Negative = enemy = move AWAY (score is already negative)
                    steering +=
                        direction * relationship_score * relationship_influence.repulsion_strength;
                }
            }
        }
    }

    steering.normalize_or_zero()
}

/// Calculate the steering force for center pull behavior
/// Returns a force vector pointing toward world origin (0, 0)
/// Strength increases exponentially as (distance/radius)^5
fn calculate_center_pull_steering(transform: &Transform) -> Vec2 {
    let pos = Vec2::new(transform.translation.x, transform.translation.y);
    let distance_from_center = pos.length();

    // Avoid division by zero at exact center
    if distance_from_center < 0.01 {
        return Vec2::ZERO;
    }

    // Calculate normalized strength: (distance / half_world)^5
    let normalized_distance = distance_from_center / WORLD_HALF_SIZE;
    let strength = normalized_distance.powf(5.0);

    // Direction points toward origin: -position.normalize()
    let direction = -pos.normalize();

    // Return steering force: direction * strength
    direction * strength
}
