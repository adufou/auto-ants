use bevy::prelude::*;
use std::collections::HashMap;

/// Component that stores a human's relationships with other humans
/// Relationship scores range from -1.0 (enemy) to 1.0 (ally), default 0.0 (neutral)
#[derive(Component, Debug, Clone, Default)]
pub struct HumanRelationships {
    /// Maps entity ID to relationship score
    /// Only stores entities this human has encountered within interaction radius
    pub relationships: HashMap<Entity, f32>,
}

impl HumanRelationships {
    /// Get relationship score with another entity (returns 0.0 if never met)
    pub fn get_relationship(&self, entity: Entity) -> f32 {
        self.relationships.get(&entity).copied().unwrap_or(0.0)
    }

    /// Update or initialize relationship with another entity
    /// Automatically clamps the score to [-1.0, 1.0]
    pub fn update_relationship(&mut self, entity: Entity, new_score: f32) {
        let clamped = new_score.clamp(-1.0, 1.0);
        self.relationships.insert(entity, clamped);
    }

    /// Returns number of entities this human has relationships with
    pub fn relationship_count(&self) -> usize {
        self.relationships.len()
    }
}
