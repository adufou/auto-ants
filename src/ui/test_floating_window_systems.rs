use super::test_floating_window::{TestFloatingWindowRoot, TestWindowCloseButton};
use bevy::prelude::*;

/// Spawns the test floating window on startup
pub fn spawn_test_window(mut commands: Commands) {
    commands.spawn((
        Node::default(),
        Visibility::default(),
        TestFloatingWindowRoot,
    ));
}

/// Handles close button interaction for the test floating window
pub fn handle_test_window_close(
    button_query: Query<&Interaction, (Changed<Interaction>, With<TestWindowCloseButton>)>,
    mut commands: Commands,
    window_query: Query<Entity, With<TestFloatingWindowRoot>>,
) {
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            // Despawn the window when close button is clicked
            for window_entity in window_query.iter() {
                commands.entity(window_entity).despawn();
            }
        }
    }
}
