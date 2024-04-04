mod menu_view;

use bevy::prelude::*;
use crate::AppState;
use crate::menu::menu_view::{check_menu_interactions, despawn_menu_ui, spawn_menu_ui};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Menu), spawn_menu_ui)
            .add_systems(Update, (check_menu_interactions,)
                .run_if(in_state(AppState::Menu)))
            .add_systems(OnExit(AppState::Menu), despawn_menu_ui);
    }
}