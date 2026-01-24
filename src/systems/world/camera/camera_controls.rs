use bevy::prelude::*;
use crate::resources::{CameraConfig, TerrainConfig};

/// Camera movement system with world bounds
pub fn camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
    camera_config: Res<CameraConfig>,
    terrain_config: Res<TerrainConfig>,
) {
    const MOVEMENT_KEYS: &[(KeyCode, f32, f32)] = &[
        (KeyCode::KeyW, 0.0, 1.0),   // Up
        (KeyCode::KeyS, 0.0, -1.0),  // Down
        (KeyCode::KeyA, -1.0, 0.0),  // Left
        (KeyCode::KeyD, 1.0, 0.0),   // Right
    ];

    // Calculate world bounds: 8 chunks * 16 tiles * 16 pixels = 2048
    let half_world_pixels = (8 * terrain_config.chunk_size * 16) as f32;

    for mut transform in &mut camera_query {
        let mut velocity = Vec3::ZERO;

        for (key, dx, dy) in MOVEMENT_KEYS {
            if keyboard.pressed(*key) {
                velocity.x += dx;
                velocity.y += dy;
            }
        }

        if velocity != Vec3::ZERO {
            transform.translation += velocity.normalize() * camera_config.movement_speed * time.delta_secs();
        }

        // Clamp camera position to world bounds
        transform.translation.x = transform.translation.x.clamp(-half_world_pixels, half_world_pixels);
        transform.translation.y = transform.translation.y.clamp(-half_world_pixels, half_world_pixels);
    }
}
