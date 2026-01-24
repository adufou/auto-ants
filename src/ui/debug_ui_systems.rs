use super::debug_ui::CheckboxState;
use bevy::prelude::*;

/// System to handle checkbox clicks
pub fn handle_checkbox_interaction(
    mut checkbox_query: Query<(&Interaction, &mut CheckboxState, &Children), Changed<Interaction>>,
    mut visibility_query: Query<&mut Visibility>,
) {
    for (interaction, mut state, children) in checkbox_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            state.checked = !state.checked;

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
