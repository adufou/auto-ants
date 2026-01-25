use bevy::prelude::*;

/// Component that pulls entities toward the world center (0, 0)
/// Strength increases exponentially with distance from center
#[derive(Component, Debug, Clone)]
pub struct CenterPullInfluence {
    pub weight: f32,
}

impl Default for CenterPullInfluence {
    fn default() -> Self {
        Self { weight: 1.0 }
    }
}
