use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod systems;

use plugins::HelloPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
