mod common;
mod game;
mod menu;
mod window;

use crate::common::better_button::BetterButtonPlugin;
use bevy::text::TextSettings;
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
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
        .insert_resource(TextSettings {
            allow_dynamic_font_size: true,
            ..default()
        })
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::KeyI)),
        )
        .add_plugins((
            BetterButtonPlugin,
            game::GamePlugin,
            menu::MenuPlugin,
            UiPlugin,
        ))
        .run();
}
