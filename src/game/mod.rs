//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

mod collisions;
mod enemy;
pub mod events;
pub mod level;
mod movement;
mod particles;
mod player;
pub mod resources;
pub mod state;
mod walls;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // required for Shape Lyon
    app.insert_resource(Msaa::Sample4).add_plugins(ShapePlugin);

    app.init_resource::<Score>();

    // Game time system
    app.init_resource::<GameTime>().add_systems(
        Update,
        (update_game_time.in_set(AppSet::TickTimers)).run_if(in_state(InGameState::Playing)),
    );

    // Physics system
    app.add_plugins((
        RapierPhysicsPlugin::<NoUserData>::default(),
        // #[cfg(debug_assertions)]
        // RapierDebugRenderPlugin::default(),
    ));

    // Game systems
    app.add_plugins((
        movement::plugin,
        player::plugin,
        level::plugin,
        enemy::plugin,
        walls::plugin,
        particles::plugin,
        collisions::plugin,
        state::plugin,
    ));
}

fn update_game_time(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    game_time.time += time.delta_seconds();
}
