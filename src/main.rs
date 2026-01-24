use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod components;
mod plugins;
mod resources;
mod systems;

use plugins::{UiPlugin, WorldPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(UiPlugin)
        .run();
}
