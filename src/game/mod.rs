mod events;
mod pig;
mod player;
mod resources;
mod state;
mod views;

use bevy::prelude::*;
use resources::*;

use crate::AppState;

use self::{
    pig::PigPlugin,
    player::{despawn_player, spawn_player, Player},
    state::{GameStatePlugin, InGameState},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // add plugins for modular management of game
            .add_plugins((GameStatePlugin, PigPlugin))
            // Initialize Game Resources
            .init_resource::<Score>()
            .init_resource::<PlayerSpeed>()
            .init_resource::<GameTime>()
            .register_type::<Player>() // used for debug inspection
            .add_systems(OnEnter(AppState::InGame), spawn_player)
            .add_systems(OnExit(AppState::InGame), despawn_player)
            .add_systems(
                Update,
                (player::character_movement, update_game_time).run_if(in_state(InGameState::Play)),
            );
    }
}

fn update_game_time(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    game_time.time += time.delta_seconds();
}
