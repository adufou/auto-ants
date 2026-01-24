use crate::components::markers::Human;
use crate::resources::{CameraConfig, SelectedHuman, TerrainConfig};
use bevy::prelude::*;

/// Camera follow system that smoothly tracks the selected human
pub fn camera_follow_human(
    selected_human: Res<SelectedHuman>,
    human_query: Query<&Transform, (With<Human>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    camera_config: Res<CameraConfig>,
    terrain_config: Res<TerrainConfig>,
    time: Res<Time>,
) {
    // Early return if no human is selected
    let Some(selected_entity) = selected_human.0 else {
        return;
    };

    // Get the selected human's transform
    let Ok(human_transform) = human_query.get(selected_entity) else {
        return;
    };

    // Calculate world bounds (same as camera_controls)
    let half_world_pixels = (8 * terrain_config.chunk_size * 16) as f32;

    for mut camera_transform in &mut camera_query {
        let target_position = human_transform.translation;

        // Smooth lerp toward target position
        // Multiply by 60.0 to make smoothing frame-rate independent (calibrated for 60fps)
        let lerp_factor = camera_config.follow_smoothing * time.delta_secs() * 60.0;
        let lerp_factor_clamped = lerp_factor.clamp(0.0, 1.0);

        camera_transform.translation.x = camera_transform.translation.x
            + (target_position.x - camera_transform.translation.x) * lerp_factor_clamped;
        camera_transform.translation.y = camera_transform.translation.y
            + (target_position.y - camera_transform.translation.y) * lerp_factor_clamped;

        // Clamp to world bounds
        camera_transform.translation.x = camera_transform
            .translation
            .x
            .clamp(-half_world_pixels, half_world_pixels);
        camera_transform.translation.y = camera_transform
            .translation
            .y
            .clamp(-half_world_pixels, half_world_pixels);
    }
}
