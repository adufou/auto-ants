use crate::components::markers::Human;
use crate::resources::SelectedHuman;
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const SELECTION_RADIUS: f32 = 16.0;

/// Converts screen coordinates to world coordinates
fn screen_to_world(
    cursor_pos: Vec2,
    camera_transform: &GlobalTransform,
    projection: &Projection,
    window: &Window,
) -> Option<Vec2> {
    let Projection::Orthographic(ortho) = projection else {
        return None;
    };

    let window_size = Vec2::new(window.width(), window.height());

    // Convert screen coordinates to NDC (normalized device coordinates)
    // Screen coordinates have origin at top-left, need to flip Y
    let ndc = ((cursor_pos / window_size) * 2.0 - Vec2::ONE) * Vec2::new(1.0, -1.0);

    // Apply orthographic projection scale and camera position
    let camera_pos = camera_transform.translation().truncate();
    let world_pos = camera_pos + ndc * window_size * 0.5 * ortho.scale;

    Some(world_pos)
}

/// Handles mouse clicks to select humans
pub fn handle_human_click(
    mouse_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform, &Projection), With<Camera2d>>,
    humans_query: Query<(Entity, &Transform), With<Human>>,
    mut selected_human: ResMut<SelectedHuman>,
) {
    // Check if left mouse button was just pressed
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    // Get the primary window
    let Ok(window) = window_query.single() else {
        return;
    };

    // Get cursor position
    let Some(cursor_pos): Option<Vec2> = window.cursor_position() else {
        return;
    };

    // Get camera data
    let Ok((_camera, camera_transform, projection)) = camera_query.single() else {
        return;
    };

    // Convert screen to world coordinates
    let Some(world_pos) = screen_to_world(cursor_pos, camera_transform, projection, window) else {
        return;
    };

    // Find the closest human within selection radius
    let mut closest: Option<(Entity, f32)> = None;

    for (entity, transform) in humans_query.iter() {
        let human_pos = transform.translation.truncate();
        let distance = world_pos.distance(human_pos);

        if distance <= SELECTION_RADIUS {
            match closest {
                Some((_, best_dist)) if distance < best_dist => {
                    closest = Some((entity, distance));
                }
                None => {
                    closest = Some((entity, distance));
                }
                _ => {}
            }
        }
    }

    // Update selected human
    if let Some((entity, _)) = closest {
        selected_human.0 = Some(entity);
    }
}
