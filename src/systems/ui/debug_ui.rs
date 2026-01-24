use crate::components::ui::debug_ui::{CheckboxState, DebugUiRoot};
use crate::config::ui::UI_PADDING;
use crate::resources::DebugConfig;
use bevy::prelude::*;

/// Spawns the debug UI in the bottom-right corner
pub fn spawn_debug_ui(mut commands: Commands) {
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

/// System to handle checkbox clicks
pub fn handle_checkbox_interaction(
    mut checkbox_query: Query<(&Interaction, &mut CheckboxState, &Children), Changed<Interaction>>,
    mut visibility_query: Query<&mut Visibility>,
    mut debug_config: ResMut<DebugConfig>,
) {
    for (interaction, mut state, children) in checkbox_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            state.checked = !state.checked;

            // Update global debug config
            debug_config.show_vision_debug = state.checked;

            // Update checkmark visibility
            if let Some(&checkmark_entity) = children.first() {
                if let Ok(mut visibility) = visibility_query.get_mut(checkmark_entity) {
                    *visibility = if state.checked {
                        Visibility::Visible
                    } else {
                        Visibility::Hidden
                    };
                }
            }
        }
    }
}
