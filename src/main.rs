mod plugins;
mod resources;
mod systems;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(plugins::app_plugins())
        .init_resource::<resources::Money>()
        .add_plugins(plugins::pig::PigPlugin)
        .add_plugins(plugins::player::PlayerPlugin)
        .add_systems(Startup, systems::startup)
        .run();
}
