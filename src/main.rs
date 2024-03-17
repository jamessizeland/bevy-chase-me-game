mod components;
mod plugins;
mod resources;
mod systems;
mod utils;
use bevy::prelude::*;
use plugins::app_plugins;

fn main() {
    App::new()
        .add_plugins(app_plugins())
        .add_systems(Startup, systems::startup)
        .add_systems(Update, systems::character_movement)
        .run();
}
