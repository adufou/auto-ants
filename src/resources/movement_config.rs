use bevy::prelude::*;

/// Global configuration for movement system
#[derive(Resource, Debug, Clone)]
pub struct MovementConfig {
    /// Base movement speed in pixels per second
    /// Applied after weight normalization
    pub base_speed: f32,
}

impl Default for MovementConfig {
    fn default() -> Self {
        Self {
            base_speed: 50.0, // Same as original random walk speed
        }
    }
}
