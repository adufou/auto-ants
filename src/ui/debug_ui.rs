use super::capabilities::*;
use bevy::prelude::*;
use bevy::ui::widget::Text;
use bevy_immediate::{Imm, attach::ImmediateAttach};

/// Root component for the debug UI in the bottom-right corner
#[derive(Component)]
pub struct DebugUiRoot;

/// Checkbox state component
#[derive(Component, Default)]
pub struct CheckboxState {
    pub checked: bool,
}

impl ImmediateAttach<CapsUi> for DebugUiRoot {
    type Params = ();

    fn construct(ui: &mut Imm<CapsUi>, _params: &mut Self::Params) {
        // Container for checkbox + label (horizontal layout)
        ui.ch()
            .on_spawn_insert(|| Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(CHECKBOX_LABEL_GAP),
                ..default()
            })
            .add(|ui| {
                // Checkbox button
                ui.ch_id("checkbox")
                    .on_spawn_insert(|| {
                        (
                            Button,
                            Node {
                                width: Val::Px(CHECKBOX_SIZE),
                                height: Val::Px(CHECKBOX_SIZE),
                                border: UiRect::all(Val::Px(CHECKBOX_BORDER_WIDTH)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(CHECKBOX_BG),
                            BorderColor::all(CHECKBOX_BORDER),
                            CheckboxState::default(),
                        )
                    })
                    .add(|ui| {
                        // Checkmark (X) - initially hidden
                        ui.ch_id("checkmark").on_spawn_insert(|| {
                            (TextColor(TEXT_COLOR), Text("X".into()), Visibility::Hidden)
                        });
                    });

                // Label text
                ui.ch().on_spawn_insert(|| {
                    (
                        TextColor(TEXT_COLOR),
                        Text("Debug".into()),
                        TextFont {
                            font_size: FONT_SIZE_NORMAL,
                            ..default()
                        },
                    )
                });
            });
    }
}
