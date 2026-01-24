use crate::ui::capabilities::UI_PADDING;
use crate::ui::{CapsUi, DebugUiRoot, handle_checkbox_interaction};
use bevy::prelude::*;
use bevy_immediate::attach::BevyImmediateAttachPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add bevy_immediate plugin for our UI
            .add_plugins(BevyImmediateAttachPlugin::<CapsUi, DebugUiRoot>::new())
            // Spawn the debug UI on startup
            .add_systems(Startup, spawn_debug_ui)
            // Add checkbox interaction system
            .add_systems(Update, handle_checkbox_interaction);
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
