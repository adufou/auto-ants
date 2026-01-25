use crate::components::{
    CenterPullInfluence, CohesionInfluence, CurrentDirection, DesiredDirection, Human,
    HumanPhysics, HumanRelationships, MovementVelocity, RandomWalkInfluence, RelationshipInfluence,
};
use crate::resources::{EntityAssets, PhysicsConfig};
use bevy::prelude::*;
use rand::random;
use std::f32::consts::PI;

/// Number of humans to spawn at game start
const NUM_HUMANS: usize = 32;

/// Radius (in pixels) of the cluster where humans spawn
const CLUSTER_RADIUS: f32 = 512.0;

/// System that spawns 32 humans clustered near world center when assets are ready
/// Uses Local<bool> to ensure humans are only spawned once
pub fn spawn_human(
    mut commands: Commands,
    entity_assets: Option<Res<EntityAssets>>,
    physics_config: Res<PhysicsConfig>,
    images: Res<Assets<Image>>,
    existing_humans: Query<Entity, With<Human>>,
    mut spawned: Local<bool>,
) {
    // Only run once
    if *spawned || !existing_humans.is_empty() {
        return;
    }

    // Wait for EntityAssets resource to exist
    let entity_assets = match entity_assets {
        Some(assets) => assets,
        None => return,
    };

    // Wait for texture to finish loading
    if !images.contains(&entity_assets.texture_handle) {
        return;
    }

    // Spawn 32 humans in a cluster near world center
    for _ in 0..NUM_HUMANS {
        // Generate random position within cluster radius using polar coordinates
        let angle = random::<f32>() * std::f32::consts::TAU;
        let distance = random::<f32>() * CLUSTER_RADIUS;
        let x = angle.cos() * distance;
        let y = angle.sin() * distance;

        commands
            .spawn((
                Sprite::from_atlas_image(
                    entity_assets.texture_handle.clone(),
                    TextureAtlas {
                        layout: entity_assets.texture_atlas_layout.clone(),
                        index: 0, // top-left sprite
                    },
                ),
                Transform::from_xyz(x, y, 1.0),
                Human,
                HumanPhysics::new(&physics_config),
            ))
            .insert((
                // Movement components
                MovementVelocity::default(),
                RandomWalkInfluence {
                    direction: Vec2::new(angle.cos(), angle.sin()),
                    direction_change_rate: 2.0 * PI,
                    weight: 2.0,
                },
                CohesionInfluence {
                    perception_radius: 256.0,
                    weight: 1.0,
                },
                RelationshipInfluence {
                    perception_radius: 256.0,
                    weight: 1.0,
                    attraction_strength: 1.0,
                    repulsion_strength: 1.0,
                },
                CenterPullInfluence { weight: 1.0 },
                CurrentDirection {
                    direction: Vec2::new(angle.cos(), angle.sin()),
                },
                DesiredDirection::default(),
                // Social components
                HumanRelationships::default(),
            ));
    }

    *spawned = true;
    info!("Spawned {} humans near world center", NUM_HUMANS);
}
