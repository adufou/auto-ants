use crate::components::DebugCheckbox;
use bevy::prelude::*;

/// Startup system that creates the debug checkbox UI in the bottom-right corner
pub fn setup_debug_ui(mut commands: Commands) {
    // Root container node (fills screen, transparent)
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexEnd, // Align to right
            align_items: AlignItems::FlexEnd,         // Align to bottom
            padding: UiRect::all(Val::Px(16.0)),
            ..default()
        })
        .with_children(|parent| {
            // Container for checkbox + label (horizontal layout)
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|parent| {
                    // Checkbox button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(20.0),
                                height: Val::Px(20.0),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
                            BorderColor::all(Color::WHITE),
                            DebugCheckbox { checked: false },
                        ))
                        .with_children(|parent| {
                            // Checkmark text (initially hidden)
                            parent.spawn((
                                Text::new("X"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                Visibility::Hidden, // Hidden when unchecked
                            ));
                        });

                    // Label text
                    parent.spawn((
                        Text::new("Debug"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}
