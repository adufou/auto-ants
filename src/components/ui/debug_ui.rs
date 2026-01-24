use bevy::prelude::*;

/// Root component for the debug UI in the bottom-right corner
#[derive(Component)]
pub struct DebugUiRoot;

/// Checkbox state component
#[derive(Component, Default)]
pub struct CheckboxState {
    pub checked: bool,
}
