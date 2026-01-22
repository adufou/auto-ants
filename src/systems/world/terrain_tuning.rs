use crate::resources::TerrainConfig;
use bevy::prelude::*;

/// System to adjust terrain parameters with keyboard controls (for Phase 3 tuning)
/// - UP/DOWN arrows: adjust water threshold (-0.1 to +0.1)
/// - LEFT/RIGHT arrows: adjust noise frequency (-0.01 to +0.01)
pub fn terrain_tuning(keyboard: Res<ButtonInput<KeyCode>>, mut config: ResMut<TerrainConfig>) {
    let threshold_delta = 0.05;
    let frequency_delta = 0.01;

    if keyboard.just_pressed(KeyCode::ArrowUp) {
        config.water_threshold += threshold_delta;
        config.water_threshold = config.water_threshold.clamp(-1.0, 1.0);
        info!("Water threshold: {:.2}", config.water_threshold);
    }

    if keyboard.just_pressed(KeyCode::ArrowDown) {
        config.water_threshold -= threshold_delta;
        config.water_threshold = config.water_threshold.clamp(-1.0, 1.0);
        info!("Water threshold: {:.2}", config.water_threshold);
    }

    if keyboard.just_pressed(KeyCode::ArrowRight) {
        config.noise_frequency += frequency_delta;
        info!("Noise frequency: {:.4}", config.noise_frequency);
    }

    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        config.noise_frequency -= frequency_delta;
        config.noise_frequency = config.noise_frequency.max(0.001);
        info!("Noise frequency: {:.4}", config.noise_frequency);
    }
}
