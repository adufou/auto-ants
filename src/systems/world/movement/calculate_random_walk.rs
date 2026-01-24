use crate::components::{Human, RandomWalkInfluence};
use bevy::prelude::*;
use rand::Rng;

/// System that updates random walk direction for entities
/// Only updates the direction vector; does NOT modify Transform
/// The actual movement is applied by the apply_movement system
pub fn calculate_random_walk(
    mut query: Query<&mut RandomWalkInfluence, With<Human>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let mut rng = rand::thread_rng();

    for mut influence in &mut query {
        // Generate random angle change between -1.0 and 1.0
        let random_value: f32 = rng.gen_range(-1.0..=1.0);
        let angle_change = random_value * influence.direction_change_rate * dt;

        // Get current angle and apply random perturbation
        let current_angle = influence.direction.y.atan2(influence.direction.x);
        let new_angle = current_angle + angle_change;

        // Update direction vector (keep it normalized)
        influence.direction = Vec2::new(new_angle.cos(), new_angle.sin());
    }
}
