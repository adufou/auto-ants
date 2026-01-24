use crate::resources::TerrainConfig;
use bevy::prelude::*;

/// System to adjust terrain parameters with keyboard controls
/// - UP/DOWN arrows: adjust water threshold (-0.1 to +0.1)
/// - LEFT/RIGHT arrows: adjust noise frequency (-0.01 to +0.01)
pub fn terrain_tuning(keyboard: Res<ButtonInput<KeyCode>>, mut config: ResMut<TerrainConfig>) {
    const THRESHOLD_DELTA: f32 = 0.05;
    const FREQUENCY_DELTA: f64 = 0.01;
    const THRESHOLD_MIN: f32 = -1.0;
    const THRESHOLD_MAX: f32 = 1.0;
    const FREQUENCY_MIN: f64 = 0.001;

    let mut threshold_changed = false;
    let mut frequency_changed = false;

    if keyboard.just_pressed(KeyCode::ArrowUp) {
        config.water_threshold += THRESHOLD_DELTA;
        threshold_changed = true;
    }

    if keyboard.just_pressed(KeyCode::ArrowDown) {
        config.water_threshold -= THRESHOLD_DELTA;
        threshold_changed = true;
    }

    if threshold_changed {
        config.water_threshold = config.water_threshold.clamp(THRESHOLD_MIN, THRESHOLD_MAX);
        info!("Water threshold: {:.2}", config.water_threshold);
    }

    if keyboard.just_pressed(KeyCode::ArrowRight) {
        config.noise_frequency += FREQUENCY_DELTA;
        frequency_changed = true;
    }

    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        config.noise_frequency -= FREQUENCY_DELTA;
        frequency_changed = true;
    }

    if frequency_changed {
        config.noise_frequency = config.noise_frequency.max(FREQUENCY_MIN);
        info!("Noise frequency: {:.4}", config.noise_frequency);
    }
}
