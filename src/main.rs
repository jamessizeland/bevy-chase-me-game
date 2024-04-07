mod common;
mod game;
mod menu;
mod window;

use crate::common::better_button::BetterButtonPlugin;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::text::TextSettings;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::plugin::ShapePlugin;
use window::UiPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    RestartInGame,
}

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Chase me!".into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(Msaa::Sample4) // shape lyon
        .insert_resource(TextSettings {
            allow_dynamic_font_size: true,
            ..default()
        })
        .add_plugins((
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::KeyI)),
        ))
        .add_plugins((
            ShapePlugin, // shape lyon
            BetterButtonPlugin,
            game::GamePlugin,
            menu::MenuPlugin,
            UiPlugin,
        ))
        .run();
}
