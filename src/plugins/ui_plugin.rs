use crate::systems::ui::{handle_checkbox_interaction, setup_debug_ui};
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_debug_ui);
        app.add_systems(Update, handle_checkbox_interaction);
    }
}
