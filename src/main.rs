mod components;
mod plugins;
mod resources;
mod systems;
mod utils;
use bevy::prelude::*;
use systems::{pig, player};

fn main() {
    App::new()
        .add_plugins(plugins::app_plugins())
        .init_resource::<resources::Money>()
        .add_systems(Startup, systems::startup)
        .add_systems(Update, (player::character_movement,))
        .add_systems(
            Update,
            (pig::spawn_pig, pig::pig_lifetime, pig::pig_movement),
        )
        .run();
}
