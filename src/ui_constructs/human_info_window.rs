use crate::components::ui::floating_window::{
    HumanInfoCloseButton, HumanInfoEntityId, HumanInfoInfluences, HumanInfoNeighbors,
    HumanInfoPosition, HumanInfoRelationships, HumanInfoVelocity, HumanInfoWindowRoot,
};
use crate::config::ui::*;
use bevy::prelude::*;
use bevy_immediate::ui::floating_window_plugin::{FloatingWindow, resizable_borders};
use bevy_immediate::{Imm, attach::ImmediateAttach};

use super::CapsUi;

impl ImmediateAttach<CapsUi> for HumanInfoWindowRoot {
    type Params = ();

    fn construct(ui: &mut Imm<CapsUi>, _params: &mut Self::Params) {
        // Create the floating window with configuration
        ui.ch()
            .on_spawn_insert(|| FloatingWindow {
                initial_width: Val::Px(350.0),
                initial_height: Val::Px(400.0),
                min_width: Val::Px(250.0),
                min_height: Val::Px(300.0),
                max_width: Val::Vw(50.0),
                max_height: Val::Vh(70.0),
                ..default()
            })
            .on_spawn_insert(|| resizable_borders(8.0, ()))
            .add(|ui| {
                // Content container
                ui.ch()
                    .on_spawn_insert(|| Node {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    })
                    .add(|ui| {
                        // Title bar (draggable area)
                        ui.ch_id("title_bar")
                            .on_spawn_insert(|| Node {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                height: Val::Px(WINDOW_TITLE_BAR_HEIGHT),
                                width: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(8.0)),
                                border: UiRect::bottom(Val::Px(1.0)),
                                ..default()
                            })
                            .on_spawn_insert(|| {
                                (
                                    BackgroundColor(WINDOW_BG_COLOR),
                                    BorderColor::all(WINDOW_BORDER_COLOR),
                                )
                            })
                            .add(|ui| {
                                // Title text
                                ui.ch().on_spawn_insert(|| {
                                    (
                                        TextColor(TEXT_COLOR),
                                        Text("Human Info".into()),
                                        TextFont {
                                            font_size: WINDOW_TITLE_TEXT_SIZE,
                                            ..default()
                                        },
                                    )
                                });

                                // Close button
                                ui.ch_id("close_button")
                                    .on_spawn_insert(|| {
                                        (
                                            Button,
                                            Node {
                                                width: Val::Px(CLOSE_BUTTON_SIZE),
                                                height: Val::Px(CLOSE_BUTTON_SIZE),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgba(0.8, 0.2, 0.2, 0.8)),
                                            HumanInfoCloseButton,
                                        )
                                    })
                                    .add(|ui| {
                                        // Close button text (X)
                                        ui.ch().on_spawn_insert(|| {
                                            (
                                                TextColor(TEXT_COLOR),
                                                Text("X".into()),
                                                TextFont {
                                                    font_size: 16.0,
                                                    ..default()
                                                },
                                            )
                                        });
                                    });
                            });

                        // Body content
                        ui.ch_id("body")
                            .on_spawn_insert(|| Node {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Start,
                                align_items: AlignItems::Start,
                                width: Val::Percent(100.0),
                                flex_grow: 1.0,
                                padding: UiRect::all(Val::Px(12.0)),
                                row_gap: Val::Px(8.0),
                                ..default()
                            })
                            .on_spawn_insert(|| BackgroundColor(WINDOW_BG_COLOR))
                            .add(|ui| {
                                // Entity ID text (small, gray)
                                ui.ch().on_spawn_insert(|| {
                                    (
                                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                                        Text("Entity: ...".into()),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        HumanInfoEntityId,
                                    )
                                });

                                // Position text
                                ui.ch().on_spawn_insert(|| {
                                    (
                                        TextColor(TEXT_COLOR),
                                        Text("Position: (0.0, 0.0)".into()),
                                        TextFont {
                                            font_size: FONT_SIZE_NORMAL,
                                            ..default()
                                        },
                                        HumanInfoPosition,
                                    )
                                });

                                // Velocity text
                                ui.ch().on_spawn_insert(|| {
                                    (
                                        TextColor(TEXT_COLOR),
                                        Text("Velocity: 0.0 px/s @ 0°".into()),
                                        TextFont {
                                            font_size: FONT_SIZE_NORMAL,
                                            ..default()
                                        },
                                        HumanInfoVelocity,
                                    )
                                });

                                // Neighbors count
                                ui.ch().on_spawn_insert(|| {
                                    (
                                        TextColor(TEXT_COLOR),
                                        Text("Nearby humans: 0".into()),
                                        TextFont {
                                            font_size: FONT_SIZE_NORMAL,
                                            ..default()
                                        },
                                        HumanInfoNeighbors,
                                    )
                                });

                                // Influences section
                                ui.ch().on_spawn_insert(|| {
                                    (
                                        TextColor(TEXT_COLOR),
                                        Text("Influences:\n  Random Walk: weight 0.00\n  Cohesion: radius 0px, weight 0.00".into()),
                                        TextFont {
                                            font_size: FONT_SIZE_NORMAL,
                                            ..default()
                                        },
                                        HumanInfoInfluences,
                                    )
                                });

                                // Relationships section
                                ui.ch().on_spawn_insert(|| {
                                    (
                                        TextColor(TEXT_COLOR),
                                        Text("Relationships: None".into()),
                                        TextFont {
                                            font_size: FONT_SIZE_NORMAL,
                                            ..default()
                                        },
                                        HumanInfoRelationships,
                                    )
                                });
                            });
                    });
            });
    }
}
