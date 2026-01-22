use bevy::prelude::*;
use std::f32::consts::PI;

/// Component for entities that exhibit random walking behavior
/// Implements continuous Brownian motion with smooth direction changes
#[derive(Component, Debug, Clone)]
pub struct RandomWalkBehavior {
    /// Movement speed in pixels per second
    pub speed: f32,
    /// Current movement direction (normalized vector)
    pub direction: Vec2,
    /// How quickly direction changes in radians per second
    pub direction_change_rate: f32,
}

impl Default for RandomWalkBehavior {
    fn default() -> Self {
        Self {
            speed: 50.0,
            direction: Vec2::new(1.0, 0.0),
            direction_change_rate: 2.0 * PI,
        }
    }
}
