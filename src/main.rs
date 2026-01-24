use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod components;
mod config;
mod plugins;
mod resources;
mod systems;
mod ui_constructs;

use plugins::{UiPlugin, WorldPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::ZERO))
        .add_plugins(WorldPlugin)
        .add_plugins(UiPlugin)
        .run();
}
