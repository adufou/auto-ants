use crate::components::{Human, RandomWalkBehavior};
use crate::resources::EntityAssets;
use bevy::prelude::*;

/// System that spawns a single human at world origin when assets are ready
/// Uses Local<bool> to ensure only one human is spawned
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

    // Spawn the human at world center (0, 0) with z=1 to render above tilemap
    commands.spawn((
        Sprite::from_atlas_image(
            entity_assets.texture_handle.clone(),
            TextureAtlas {
                layout: entity_assets.texture_atlas_layout.clone(),
                index: 0, // top-left sprite
            },
        ),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Human,
        RandomWalkBehavior::default(),
    ));

    *spawned = true;
    info!("Spawned human at world origin (0, 0)");
}
