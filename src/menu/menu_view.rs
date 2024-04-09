use crate::common::better_button::ReleaseButton;
use crate::common::styles::{get_full_screen_menu_node_bundle, spawn_full_screen_menu_button};
use crate::AppState;
use bevy::prelude::*;

#[derive(Component)]
pub struct MenuView;
#[derive(Component, Default)]
pub struct PlayButton;
#[derive(Component, Default)]
pub struct OptionsButton;

pub fn spawn_menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((MenuView {}, get_full_screen_menu_node_bundle()))
        .with_children(|builder| {
            spawn_full_screen_menu_button::<PlayButton>(
                builder,
                &asset_server,
                "Play",
                KeyCode::KeyP,
            );
        });
}

pub fn despawn_menu_ui(mut commands: Commands, root_query: Query<Entity, With<MenuView>>) {
    if let Ok(root) = root_query.get_single() {
        commands.entity(root).despawn_recursive();
    }
}

pub fn check_menu_interactions(
    play_button_query: Query<&ReleaseButton, With<PlayButton>>,
    // options_button_query: Query<&ReleaseButton, With<OptionsButton>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for button in play_button_query.iter() {
        if button.just_released {
            next_state.set(AppState::InGame);
            return;
        }
    }
}
