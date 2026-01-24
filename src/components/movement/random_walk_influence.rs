use bevy::prelude::*;
use std::f32::consts::PI;

/// Component that influences movement with random wandering behavior
/// Updates direction randomly over time, contributing to the final velocity
#[derive(Component, Debug, Clone)]
pub struct RandomWalkInfluence {
    /// Current direction (normalized vector)
    pub direction: Vec2,
    /// How quickly the direction changes (radians per second)
    pub direction_change_rate: f32,
    /// Weight of this influence in the final velocity calculation (baseline: 1.0)
    pub weight: f32,
}

impl Default for RandomWalkInfluence {
    fn default() -> Self {
        Self {
            direction: Vec2::new(1.0, 0.0),
            direction_change_rate: 2.0 * PI,
            weight: 1.0, // Baseline weight
        }
    }
}
