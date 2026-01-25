use bevy::prelude::*;

/// Component that influences movement based on relationships with other entities
/// Pulls the entity toward allies (positive relationships) and away from enemies (negative relationships)
#[derive(Component, Debug, Clone)]
pub struct RelationshipInfluence {
    /// How far to consider relationships for movement (in pixels)
    pub perception_radius: f32,
    /// Weight of this influence in final velocity calculation
    pub weight: f32,
    /// Strength multiplier for attraction to allies (positive relationships)
    /// Higher values = stronger pull toward allies
    pub attraction_strength: f32,
    /// Strength multiplier for repulsion from enemies (negative relationships)
    /// Higher values = stronger push away from enemies
    pub repulsion_strength: f32,
}

impl Default for RelationshipInfluence {
    fn default() -> Self {
        Self {
            perception_radius: 256.0, // Match cohesion for consistency
            weight: 1.0,              // Equal weight with other influences
            attraction_strength: 1.0, // 1:1 relationship to movement
            repulsion_strength: 1.0,  // 1:1 relationship to movement
        }
    }
}
