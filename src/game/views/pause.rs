use crate::common::better_button::ReleaseButton;
use crate::common::styles::{
    get_full_screen_menu_node_bundle, spawn_full_screen_menu_button, spawn_full_screen_menu_header,
};
use crate::game::events::{
    MenuRequested, OptionsRequested, RestartRequested, TogglePauseRequested,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct PauseView;

#[derive(Component, Default)]
pub struct ContinueButton;

#[derive(Component, Default)]
pub struct RestartButton;

#[derive(Component, Default)]
pub struct MenuButton;

#[derive(Component, Default)]
pub struct OptionsButton;

pub fn spawn_pause_view(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((PauseView {}, get_full_screen_menu_node_bundle()))
        .with_children(|parent| {
            spawn_full_screen_menu_header(parent, &asset_server, "Pause");
            spawn_full_screen_menu_button::<ContinueButton>(
                parent,
                &asset_server,
                "Continue",
                KeyCode::KeyC,
            );
            spawn_full_screen_menu_button::<RestartButton>(
                parent,
                &asset_server,
                "Restart",
                KeyCode::KeyR,
            );
            spawn_full_screen_menu_button::<MenuButton>(
                parent,
                &asset_server,
                "Menu",
                KeyCode::KeyM,
            );
            spawn_full_screen_menu_button::<OptionsButton>(
                parent,
                &asset_server,
                "Options",
                KeyCode::KeyO,
            );
        });
}

pub fn despawn_pause_view(mut commands: Commands, view_query: Query<Entity, With<PauseView>>) {
    for view in view_query.iter() {
        commands.entity(view).despawn_recursive();
    }
}

pub fn check_pause_interactions(
    menu_button_query: Query<&ReleaseButton, With<MenuButton>>,
    restart_button_query: Query<&ReleaseButton, With<RestartButton>>,
    continue_button_query: Query<&ReleaseButton, With<ContinueButton>>,
    options_button_query: Query<&ReleaseButton, With<OptionsButton>>,
    mut menu_requested_events: EventWriter<MenuRequested>,
    mut options_requested_events: EventWriter<OptionsRequested>,
    mut restart_requested_events: EventWriter<RestartRequested>,
    mut toggle_pause_requested_events: EventWriter<TogglePauseRequested>,
) {
    for button in menu_button_query.iter() {
        if button.just_released {
            menu_requested_events.send_default();
            return;
        }
    }

    for button in restart_button_query.iter() {
        if button.just_released {
            restart_requested_events.send_default();
            return;
        }
    }

    for button in continue_button_query.iter() {
        if button.just_released {
            toggle_pause_requested_events.send_default();
            return;
        }
    }

    for button in options_button_query.iter() {
        if button.just_released {
            options_requested_events.send_default();
            return;
        }
    }
}
