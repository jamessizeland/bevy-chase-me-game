use crate::common::better_button::ReleaseButton;
use crate::common::styles::{
    get_full_screen_menu_node_bundle, spawn_full_screen_menu_button, spawn_full_screen_menu_header,
};
use crate::game::events::{TogglePauseRequested, UpdateOptions};
use bevy::prelude::*;

#[derive(Component)]
pub struct OptionsView;
#[derive(Component, Default)]
pub struct AcceptButton;
#[derive(Component, Default)]
pub struct CancelButton;

pub fn spawn_options_view(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((OptionsView {}, get_full_screen_menu_node_bundle()))
        .with_children(|parent| {
            spawn_full_screen_menu_header(parent, &asset_server, "Options");
            spawn_full_screen_menu_button::<AcceptButton>(
                parent,
                &asset_server,
                "Accept",
                KeyCode::Enter,
            );
            spawn_full_screen_menu_button::<CancelButton>(
                parent,
                &asset_server,
                "Cancel",
                KeyCode::KeyC,
            );
        });
}

pub fn despawn_options_view(mut commands: Commands, view_query: Query<Entity, With<OptionsView>>) {
    for view in view_query.iter() {
        commands.entity(view).despawn_recursive();
    }
}

pub fn check_options_interactions(
    accept_button_query: Query<&ReleaseButton, With<AcceptButton>>,
    cancel_button_query: Query<&ReleaseButton, With<CancelButton>>,
    mut update_options_requested_events: EventWriter<UpdateOptions>,
    mut toggle_pause_requested_events: EventWriter<TogglePauseRequested>,
) {
    for button in accept_button_query.iter() {
        if button.just_released {
            update_options_requested_events.send_default();
            toggle_pause_requested_events.send_default();
            return;
        }
    }
    for button in cancel_button_query.iter() {
        if button.just_released {
            toggle_pause_requested_events.send_default();
            return;
        }
    }
}
