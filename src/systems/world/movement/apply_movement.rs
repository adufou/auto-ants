use crate::components::{Human, MovementVelocity};
use bevy::prelude::*;

/// System that applies the final combined velocity to entity transforms
/// This is the final step in the movement pipeline, actually moving entities
pub fn apply_movement(
    mut query: Query<(&mut Transform, &MovementVelocity), With<Human>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.velocity.x * dt;
        transform.translation.y += velocity.velocity.y * dt;
    }
}
