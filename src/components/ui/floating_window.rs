use bevy::prelude::*;

/// Root component for the test floating window
#[derive(Component)]
pub struct TestFloatingWindowRoot;

/// Marker component for the close button
#[derive(Component)]
pub struct TestWindowCloseButton;
