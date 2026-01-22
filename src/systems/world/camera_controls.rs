use bevy::prelude::*;

/// Simple camera movement system for testing chunk loading
pub fn camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
) {
    let speed = 200.0; // pixels per second

    for mut transform in &mut camera_query {
        let mut velocity = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyW) {
            velocity.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            velocity.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            velocity.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            velocity.x += 1.0;
        }

        if velocity != Vec3::ZERO {
            transform.translation += velocity.normalize() * speed * time.delta_secs();
        }
    }
}
