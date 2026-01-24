use bevy::prelude::*;
use std::f32::consts::PI;

/// Global configuration for movement system
#[derive(Resource, Debug, Clone)]
pub struct MovementConfig {
    /// Base movement speed in pixels per second
    /// Applied after weight normalization
    pub base_speed: f32,
    /// Maximum rotation rate in radians per second
    /// Controls how fast entities can turn
    pub max_rotation_rate: f32,
}

impl Default for MovementConfig {
    fn default() -> Self {
        Self {
            base_speed: 50.0, // Same as original random walk speed
            max_rotation_rate: PI / 2.0, // 90° per second
        }
    }
}
