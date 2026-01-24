use crate::components::ui::{DebugUiRoot, TestFloatingWindowRoot};
use crate::systems::ui::{
    handle_checkbox_interaction, handle_test_window_close, spawn_debug_ui, spawn_test_window,
};
use crate::ui_constructs::CapsUi;
use bevy::prelude::*;
use bevy_immediate::attach::BevyImmediateAttachPlugin;
use bevy_immediate::ui::{
    floating_ui_focus_plugin::FloatingUiFocusPlugin, floating_window_plugin::FloatingWindowPlugin,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add floating window plugins
            .add_plugins(FloatingWindowPlugin)
            .add_plugins(FloatingUiFocusPlugin)
            // Add bevy_immediate plugins for our UI
            .add_plugins(BevyImmediateAttachPlugin::<CapsUi, DebugUiRoot>::new())
            .add_plugins(BevyImmediateAttachPlugin::<CapsUi, TestFloatingWindowRoot>::new())
            // Spawn the UI on startup
            .add_systems(Startup, (spawn_debug_ui, spawn_test_window))
            // Add interaction systems
            .add_systems(
                Update,
                (handle_checkbox_interaction, handle_test_window_close),
            );
    }
}
