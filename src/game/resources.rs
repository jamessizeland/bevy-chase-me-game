//! # Resources
//! Entities and Components are great for representing complex, query-able groups of data.
//! But most Apps will also require "globally unique" data of some kind.
//! In Bevy ECS, we represent globally unique data using Resources.
//!
//! Resources are data that is shared globally across the App.
//! Resources are stored in a World and can be accessed from any System.
//! Resources are also automatically synchronized across threads when using Bevy's parallel systems.

use crate::prelude::*;

#[derive(Resource)]
pub struct Score(pub f32);

impl Default for Score {
    fn default() -> Self {
        Score(0.0)
    }
}

/// Time in the game
#[derive(Resource)]
pub struct GameTime {
    pub time: f32,
}

impl Default for GameTime {
    fn default() -> Self {
        GameTime { time: 0.0 }
    }
}
