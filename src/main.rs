mod plugins;
mod resources;
mod systems;
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(plugins::app_plugins())
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .init_resource::<resources::Money>()
        .add_plugins(plugins::pig::PigPlugin)
        .add_plugins(plugins::player::PlayerPlugin)
        .add_systems(Startup, systems::startup)
        .run();
}
