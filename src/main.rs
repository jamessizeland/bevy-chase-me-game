mod components;
mod plugins;
mod systems;
mod utils;
use bevy::prelude::*;
use plugins::app_plugins;

// #[derive(Debug, Component)]
// struct Position {
//     x: f32,
//     y: f32,
// }

fn main() {
    App::new()
        .add_plugins(app_plugins())
        .add_systems(Startup, systems::startup)
        .add_systems(Update, systems::character_movement)
        .run();
}
