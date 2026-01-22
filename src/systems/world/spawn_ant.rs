use crate::components::{Ant, RandomWalkBehavior};
use crate::resources::EntityAssets;
use bevy::prelude::*;

/// System that spawns a single ant at world origin when assets are ready
/// Uses Local<bool> to ensure only one ant is spawned
pub fn spawn_ant(
    mut commands: Commands,
    entity_assets: Option<Res<EntityAssets>>,
    images: Res<Assets<Image>>,
    existing_ants: Query<Entity, With<Ant>>,
    mut spawned: Local<bool>,
) {
    // Only run once
    if *spawned || !existing_ants.is_empty() {
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

    // Spawn the ant at world center (0, 0) with z=1 to render above tilemap
    commands.spawn((
        Sprite::from_atlas_image(
            entity_assets.texture_handle.clone(),
            TextureAtlas {
                layout: entity_assets.texture_atlas_layout.clone(),
                index: 0, // top-left sprite
            },
        ),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Ant,
        RandomWalkBehavior::default(),
    ));

    *spawned = true;
    info!("Spawned ant at world origin (0, 0)");
}
