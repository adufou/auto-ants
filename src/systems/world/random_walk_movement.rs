use crate::components::{Ant, RandomWalkBehavior};
use bevy::prelude::*;
use rand::Rng;

/// System that updates entities with random walk behavior
/// Implements continuous Brownian motion with smooth direction changes
pub fn random_walk_movement(
    mut query: Query<(&mut Transform, &mut RandomWalkBehavior), With<Ant>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let mut rng = rand::thread_rng();

    for (mut transform, mut behavior) in &mut query {
        // Generate random angle change between -1.0 and 1.0
        let random_value: f32 = rng.gen_range(-1.0..=1.0);
        let angle_change = random_value * behavior.direction_change_rate * dt;

        // Get current angle and apply random perturbation
        let current_angle = behavior.direction.y.atan2(behavior.direction.x);
        let new_angle = current_angle + angle_change;

        // Update direction vector (keep it normalized)
        behavior.direction = Vec2::new(new_angle.cos(), new_angle.sin());

        // Move in current direction
        let velocity = behavior.direction * behavior.speed;
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }
}
