use crate::ui::capabilities::UI_PADDING;
use crate::ui::{
    CapsUi, DebugUiRoot, TestFloatingWindowRoot, handle_checkbox_interaction,
    handle_test_window_close, spawn_test_window,
};
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

/// Spawns the debug UI in the bottom-right corner
fn spawn_debug_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::FlexEnd,
            padding: UiRect::all(Val::Px(UI_PADDING)),
            ..default()
        },
        DebugUiRoot,
    ));
}
