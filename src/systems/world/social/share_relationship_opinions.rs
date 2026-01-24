use crate::components::{Human, HumanRelationships};
use crate::resources::SpatialGrid;
use crate::systems::world::social::relationship_utils::{
    apply_tanh_modifier, calculate_topic_weight,
};
use bevy::prelude::*;
use rand::Rng;

/// Distance threshold for opinion sharing (in pixels)
/// Matches RELATIONSHIP_RADIUS for consistency
const OPINION_SHARING_RADIUS: f32 = 64.0;

/// Base transmission amount before modifier application
const BASE_TRANSMISSION_AMOUNT: f32 = 0.05;

/// System where humans share their opinions about others with nearby humans.
/// Creates gossip dynamics where strong relationships (love/hate) spread as reputation.
///
/// For each pair of nearby humans (speaker and listener):
/// - Speaker selects one of their relationships to discuss (weighted by strength)
/// - Speaker's opinion influences listener's relationship with that person
/// - Transmission amount: 0.05 * apply_tanh_modifier(speaker's opinion, 0.05)
pub fn share_relationship_opinions(
    mut humans: Query<(Entity, &Transform, &mut HumanRelationships), With<Human>>,
    spatial_grid: Res<SpatialGrid>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let mut rng = rand::thread_rng();

    // Collect updates to apply after iteration (avoid double-borrow)
    let mut updates: Vec<(Entity, Entity, f32)> = Vec::new();

    // Iterate through all humans as potential speakers
    for (speaker_entity, speaker_transform, speaker_relationships) in &humans {
        let speaker_pos = Vec2::new(
            speaker_transform.translation.x,
            speaker_transform.translation.y,
        );
        let neighbors = spatial_grid.query_neighbors(speaker_pos, OPINION_SHARING_RADIUS);

        // For each nearby human (listener)
        for listener_entity in neighbors {
            // Skip self
            if listener_entity == speaker_entity {
                continue;
            }

            // Verify distance is actually within radius (spatial grid returns approximation)
            if let Ok((_, listener_transform, _)) = humans.get(listener_entity) {
                let listener_pos = Vec2::new(
                    listener_transform.translation.x,
                    listener_transform.translation.y,
                );
                let distance = speaker_pos.distance(listener_pos);

                if distance <= OPINION_SHARING_RADIUS && distance > 0.01 {
                    // Select a topic (relationship) to discuss
                    if let Some(topic_entity) = select_weighted_topic(
                        speaker_entity,
                        listener_entity,
                        &speaker_relationships,
                        &mut rng,
                    ) {
                        // Get speaker's opinion about the topic
                        let speaker_opinion = speaker_relationships.get_relationship(topic_entity);

                        // Calculate transmission amount with tanh modifier
                        // Base transmission should carry the sign of the speaker's opinion
                        let base_transmission = speaker_opinion * BASE_TRANSMISSION_AMOUNT;
                        let modified_transmission =
                            apply_tanh_modifier(speaker_opinion, base_transmission);

                        // Scale by delta time for frame-rate independence
                        let transmission_amount = modified_transmission * dt;

                        // Queue update for listener's relationship with topic
                        updates.push((listener_entity, topic_entity, transmission_amount));
                    }
                }
            }
        }
    }

    // Apply all updates
    for (listener_entity, topic_entity, transmission_amount) in updates {
        if let Ok((_, _, mut listener_relationships)) = humans.get_mut(listener_entity) {
            let current = listener_relationships.get_relationship(topic_entity);
            let new_value = current + transmission_amount;
            listener_relationships.update_relationship(topic_entity, new_value);
        }
    }
}

/// Selects a topic entity for discussion using weighted random selection
/// based on absolute relationship strength (tanh(2x) weighting).
///
/// Returns None if no valid topics exist (speaker has no relationships,
/// or only knows speaker/listener themselves).
///
/// Stronger opinions (positive or negative) are more likely to be discussed.
fn select_weighted_topic(
    speaker: Entity,
    listener: Entity,
    speaker_relationships: &HumanRelationships,
    rng: &mut impl Rng,
) -> Option<Entity> {
    // Filter valid topics: exclude speaker and listener
    let candidates: Vec<(Entity, f32)> = speaker_relationships
        .relationships
        .iter()
        .filter_map(|(entity, relationship)| {
            if *entity != speaker && *entity != listener {
                let weight = calculate_topic_weight(*relationship);
                Some((*entity, weight))
            } else {
                None
            }
        })
        .collect();

    // Edge case: no valid topics
    if candidates.is_empty() {
        return None;
    }

    // Calculate total weight
    let total_weight: f32 = candidates.iter().map(|(_, w)| w).sum();

    // Edge case: all weights are zero (shouldn't happen with tanh, but defensive)
    if total_weight <= 0.0001 {
        // Fallback to uniform random
        return Some(candidates[rng.gen_range(0..candidates.len())].0);
    }

    // Weighted random selection
    let mut random_value = rng.gen_range(0.0..total_weight);
    for (entity, weight) in &candidates {
        random_value -= weight;
        if random_value <= 0.0 {
            return Some(*entity);
        }
    }

    // Fallback (due to floating point precision)
    Some(candidates.last().unwrap().0)
}
