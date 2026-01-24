use crate::components::{Human, MovementVelocity};
use avian2d::prelude::*;
use bevy::prelude::*;

/// Force multiplier to convert desired velocity to force magnitude
/// Higher values = more responsive movement toward target velocity
/// With MaxLinearSpeed clamping, this can be increased for faster response
const FORCE_MULTIPLIER: f32 = 16.0;

/// System that applies our movement AI as forces using Avian's Forces QueryData
/// Dynamic rigid bodies then resolve collisions while respecting our intent
pub fn apply_movement(mut query: Query<(&MovementVelocity, Forces), With<Human>>) {
    for (movement_velocity, mut forces) in &mut query {
        // Apply force using Avian's QueryData API
        // Force = desired_velocity * multiplier
        forces.apply_force(movement_velocity.velocity * FORCE_MULTIPLIER);
    }
}
