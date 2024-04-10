use crate::common::better_button::ReleaseButton;
use crate::common::styles::{
    get_full_screen_menu_node_bundle, spawn_full_screen_menu_button, spawn_full_screen_menu_header,
};
use crate::game::events::{MenuRequested, RestartRequested};
use crate::game::resources::Score;
use bevy::prelude::*;
use rand::distributions::Uniform;
use rand::Rng as _;

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
    // list of sarcastic messages to display when the player gets no points
    let zero_points_message = vec![
        "Wow, you're really good at this game!".to_string(),
        "You're a natural!".to_string(),
        "Are you trying?".to_string(),
        "Seems unlikely...".to_string(),
        "Again?".to_string(),
        "Try not getting hit next time!".to_string(),
        "Try not to die!".to_string(),
        "Again?".to_string(),
        "Once more, with feeling!".to_string(),
        "Fly casual!".to_string(),
        "When life gives you lemons...".to_string(),
        "Time to git gud!".to_string(),
        "What happened?".to_string(),
        "Oh no happened!".to_string(),
        "Give it another go!".to_string(),
        "not like this...".to_string(),
        "Ouch...".to_string(),
        "I think you can do better!".to_string(),
        "You're better than this!".to_string(),
        "You can do it!".to_string(),
        "You're a star!".to_string(),
        "You're a shooting star!".to_string(),
        "You've got the power!".to_string(),
    ];
    let dist = Uniform::new(0, zero_points_message.len());
    let mut rng = rand::thread_rng();

    commands
        .spawn((SummaryView {}, get_full_screen_menu_node_bundle()))
        .with_children(|parent| {
            spawn_full_screen_menu_header(
                parent,
                &asset_server,
                format!("Your score: {}", score.0),
            );
            if score.0 == 0.0 {
                let message = &zero_points_message[rng.sample(&dist)];
                spawn_full_screen_menu_header(parent, &asset_server, message);
            }
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
