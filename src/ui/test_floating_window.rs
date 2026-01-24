use super::capabilities::*;
use bevy::prelude::*;
use bevy::ui::widget::Text;
use bevy_immediate::ui::floating_window_plugin::{FloatingWindow, resizable_borders};
use bevy_immediate::{Imm, attach::ImmediateAttach};

/// Root component for the test floating window
#[derive(Component)]
pub struct TestFloatingWindowRoot;

/// Marker component for the close button
#[derive(Component)]
pub struct TestWindowCloseButton;

impl ImmediateAttach<CapsUi> for TestFloatingWindowRoot {
    type Params = ();

    fn construct(ui: &mut Imm<CapsUi>, _params: &mut Self::Params) {
        // Create the floating window with configuration
        ui.ch()
            .on_spawn_insert(|| FloatingWindow {
                initial_width: Val::Px(300.0),
                initial_height: Val::Px(200.0),
                min_width: Val::Px(200.0),
                min_height: Val::Px(150.0),
                max_width: Val::Vw(50.0),
                max_height: Val::Vh(50.0),
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
                                        Text("Test Window".into()),
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
                                            TestWindowCloseButton,
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
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.0),
                                flex_grow: 1.0,
                                ..default()
                            })
                            .on_spawn_insert(|| BackgroundColor(WINDOW_BG_COLOR))
                            .add(|ui| {
                                // Test text
                                ui.ch().on_spawn_insert(|| {
                                    (
                                        TextColor(TEXT_COLOR),
                                        Text("Test".into()),
                                        TextFont {
                                            font_size: FONT_SIZE_NORMAL,
                                            ..default()
                                        },
                                    )
                                });
                            });
                    });
            });
    }
}
