use bevy::prelude::*;

/// Component that influences movement with cohesion behavior
/// Pulls the entity toward the center of mass of nearby entities
#[derive(Component, Debug, Clone)]
pub struct CohesionInfluence {
    /// How far to look for neighbors in pixels
    pub perception_radius: f32,
    /// Weight of this influence in the final velocity calculation (baseline: 1.0)
    pub weight: f32,
}

impl Default for CohesionInfluence {
    fn default() -> Self {
        Self {
            perception_radius: 150.0, // Look for neighbors within 150px
            weight: 1.0,              // Baseline weight
        }
    }
}
