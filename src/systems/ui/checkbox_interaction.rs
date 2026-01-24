use crate::components::DebugCheckbox;
use bevy::prelude::*;

/// System that handles checkbox interaction and updates the visual state
pub fn handle_checkbox_interaction(
    mut checkbox_query: Query<(&Interaction, &mut DebugCheckbox, &Children), Changed<Interaction>>,
    mut visibility_query: Query<&mut Visibility>,
) {
    for (interaction, mut checkbox, children) in checkbox_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // Toggle the checkbox state
            checkbox.checked = !checkbox.checked;

            // Update the checkmark visibility
            // The checkmark is the first (and only) child of the button
            if let Some(&checkmark_entity) = children.first() {
                if let Ok(mut visibility) = visibility_query.get_mut(checkmark_entity) {
                    *visibility = if checkbox.checked {
                        Visibility::Visible
                    } else {
                        Visibility::Hidden
                    };
                }
            }
        }
    }
}
