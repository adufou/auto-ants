use crate::components::{
    CohesionInfluence, CurrentDirection, DesiredDirection, Human, MovementVelocity,
    RandomWalkInfluence,
};
use crate::resources::EntityAssets;
use avian2d::prelude::*;
use bevy::prelude::*;
use rand::random;
use std::f32::consts::PI;

/// Number of humans to spawn at game start
const NUM_HUMANS: usize = 32;

/// Radius (in pixels) of the cluster where humans spawn
const CLUSTER_RADIUS: f32 = 512.0;

/// Collider radius in pixels (sprites are 16x16, use radius 7 for ~14px diameter)
const HUMAN_COLLIDER_RADIUS: f32 = 7.0;

/// Mass for each human in physics simulation (affects inertia)
const HUMAN_MASS: f32 = 1.0;

/// Linear damping (prevents infinite acceleration, works with forces to reach equilibrium)
const HUMAN_DAMPING: f32 = 2.0;

/// Maximum linear speed in pixels per second (matches base_speed from MovementConfig)
const MAX_LINEAR_SPEED: f32 = 50.0;

/// System that spawns 32 humans clustered near world center when assets are ready
/// Uses Local<bool> to ensure humans are only spawned once
pub fn spawn_human(
    mut commands: Commands,
    entity_assets: Option<Res<EntityAssets>>,
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
                // Physics components (Avian)
                RigidBody::Dynamic,
                Collider::circle(HUMAN_COLLIDER_RADIUS),
                Mass(HUMAN_MASS),
                LinearVelocity::ZERO,
                LinearDamping(HUMAN_DAMPING),
                AngularDamping(10.0),
                LockedAxes::ROTATION_LOCKED,
                Friction::ZERO,
                Restitution::ZERO,
                MaxLinearSpeed(MAX_LINEAR_SPEED),
            ))
            .insert((
                // Movement components
                MovementVelocity::default(),
                RandomWalkInfluence {
                    direction: Vec2::new(angle.cos(), angle.sin()),
                    direction_change_rate: 2.0 * PI,
                    weight: 1.0,
                },
                CohesionInfluence {
                    perception_radius: 256.0,
                    weight: 1.0,
                },
                CurrentDirection {
                    direction: Vec2::new(angle.cos(), angle.sin()),
                },
                DesiredDirection::default(),
            ));
    }

    *spawned = true;
    info!("Spawned {} humans near world center", NUM_HUMANS);
}
