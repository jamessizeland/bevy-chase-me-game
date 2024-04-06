mod events;
// mod pig;
mod movement;
mod player;
mod resources;
mod state;
mod views;
mod walls;

use self::{
    // pig::PigPlugin,
    movement::MovementPlugin,
    player::PlayerPlugin,
    state::{GameStatePlugin, InGameState},
    walls::WallsPlugin,
};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use resources::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // add plugins for modular management of game
            .add_plugins((
                GameStatePlugin,
                PlayerPlugin,
                // PigPlugin,
                WallsPlugin,
                MovementPlugin,
                PhysicsPlugins::default(),
                PhysicsDebugPlugin::default(),
            ))
            // Initialize Game Resources
            .init_resource::<Score>()
            .init_resource::<GameTime>()
            .insert_resource(Gravity::ZERO)
            .add_systems(
                Update,
                (update_game_time).run_if(in_state(InGameState::Play)),
            );
    }
}

fn update_game_time(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    game_time.time += time.delta_seconds();
}
