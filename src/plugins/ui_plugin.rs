use crate::components::ui::{DebugUiRoot, HumanInfoWindowRoot, TestFloatingWindowRoot};
use crate::resources::SelectedHuman;
use crate::systems::ui::{
    handle_checkbox_interaction, handle_human_click, handle_human_info_close,
    handle_test_window_close, spawn_debug_ui, spawn_human_info_window, spawn_test_window,
    update_human_info_content, validate_selected_human,
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
            // Add resources
            .insert_resource(SelectedHuman::default())
            // Add floating window plugins
            .add_plugins(FloatingWindowPlugin)
            .add_plugins(FloatingUiFocusPlugin)
            // Add bevy_immediate plugins for our UI
            .add_plugins(BevyImmediateAttachPlugin::<CapsUi, DebugUiRoot>::new())
            .add_plugins(BevyImmediateAttachPlugin::<CapsUi, TestFloatingWindowRoot>::new())
            .add_plugins(BevyImmediateAttachPlugin::<CapsUi, HumanInfoWindowRoot>::new())
            // Spawn the UI on startup
            .add_systems(Startup, (spawn_debug_ui, spawn_test_window))
            // Add interaction systems
            .add_systems(
                Update,
                (
                    handle_checkbox_interaction,
                    handle_test_window_close,
                    handle_human_click,
                    validate_selected_human.after(handle_human_click),
                    spawn_human_info_window.after(validate_selected_human),
                    update_human_info_content.after(spawn_human_info_window),
                    handle_human_info_close,
                ),
            );
    }
}
