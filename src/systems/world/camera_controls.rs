use bevy::prelude::*;

/// Simple camera movement system for testing chunk loading
pub fn camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 200.0; // pixels per second
    const MOVEMENT_KEYS: &[(KeyCode, f32, f32)] = &[
        (KeyCode::KeyW, 0.0, 1.0),   // Up
        (KeyCode::KeyS, 0.0, -1.0),  // Down
        (KeyCode::KeyA, -1.0, 0.0),  // Left
        (KeyCode::KeyD, 1.0, 0.0),   // Right
    ];

    for mut transform in &mut camera_query {
        let mut velocity = Vec3::ZERO;

        for (key, dx, dy) in MOVEMENT_KEYS {
            if keyboard.pressed(*key) {
                velocity.x += dx;
                velocity.y += dy;
            }
        }

        if velocity != Vec3::ZERO {
            transform.translation += velocity.normalize() * SPEED * time.delta_secs();
        }
    }
}
