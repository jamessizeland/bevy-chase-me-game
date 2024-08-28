//! Spawn the main level.

use super::player::SpawnPlayer;
use crate::prelude::*;
use bevy::ecs::world::Command;

pub(super) fn plugin(_app: &mut App) {
    // No setup required for this plugin.
    // It's still good to have a function here so that we can add some setup
    // later if needed.
}

#[derive(Debug)]
pub struct SpawnLevel;

impl Command for SpawnLevel {
    fn apply(self, world: &mut World) {
        // The only thing we have in our level is a player,
        // but add things like walls etc. here.
        world.commands().add(SpawnPlayer {
            max_speed: 8.0,
            mass: 1.3,
            thrust: 15.0,
        });

        // Flush the commands we just added so that they are
        // all executed now, as part of this command.
        world.flush_commands();
    }
}
