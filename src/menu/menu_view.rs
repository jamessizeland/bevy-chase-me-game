use bevy::app::AppExit;
use bevy::prelude::*;
use crate::AppState;
use crate::common::better_button::ReleaseButton;
use crate::common::styles::{get_full_screen_menu_node_bundle, spawn_full_screen_menu_button};

#[derive(Component)]
pub struct MenuView;
#[derive(Component, Default)]
pub struct PlayButton;
#[derive(Component, Default)]
pub struct QuitButton;

pub fn spawn_menu_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        MenuView {},
        get_full_screen_menu_node_bundle(),
    )).with_children(|builder| {
        spawn_full_screen_menu_button::<PlayButton>(builder, &asset_server, "Play", KeyCode::KeyP);
        spawn_full_screen_menu_button::<QuitButton>(builder, &asset_server, "Quit", KeyCode::KeyQ);
    });
}

pub fn despawn_menu_ui(
    mut commands: Commands,
    root_query: Query<Entity, With<MenuView>>
) {
    if let Ok(root) = root_query.get_single() {
        commands.entity(root).despawn_recursive();
    }
}

pub fn check_menu_interactions(
    play_button_query: Query<&ReleaseButton, With<PlayButton>>,
    quit_button_query: Query<&ReleaseButton, With<QuitButton>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    for button in play_button_query.iter() {
        if button.just_released {
            next_state.set(AppState::InGame);
            return;
        }
    }

    for button in quit_button_query.iter() {
        if button.just_released {
            app_exit_event_writer.send(AppExit);
            return;
        }
    }
}