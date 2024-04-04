use crate::common::better_button::ReleaseButton;
use crate::common::styles::{
    get_full_screen_menu_node_bundle, spawn_full_screen_menu_button, spawn_full_screen_menu_header,
};
use crate::game::events::{MenuRequested, RestartRequested};
use crate::game::resources::Score;
use bevy::prelude::*;

#[derive(Component)]
pub struct SummaryView;
#[derive(Component, Default)]
pub struct RestartButton;
#[derive(Component, Default)]
pub struct MenuButton;

pub fn spawn_summary_view(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    commands
        .spawn((SummaryView {}, get_full_screen_menu_node_bundle()))
        .with_children(|parent| {
            spawn_full_screen_menu_header(
                parent,
                &asset_server,
                format!("Your score: {}", score.0),
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
        });
}

pub fn despawn_summary_view(mut commands: Commands, view_query: Query<Entity, With<SummaryView>>) {
    for view in view_query.iter() {
        commands.entity(view).despawn_recursive();
    }
}

pub fn check_summary_interactions(
    menu_button_query: Query<&ReleaseButton, With<MenuButton>>,
    restart_button_query: Query<&ReleaseButton, With<RestartButton>>,
    mut menu_requested_events: EventWriter<MenuRequested>,
    mut restart_requested_events: EventWriter<RestartRequested>,
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
}
