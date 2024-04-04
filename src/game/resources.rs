//! # Resources
//! Entities and Components are great for representing complex, query-able groups of data. But most Apps will also require "globally unique" data of some kind. In Bevy ECS, we represent globally unique data using Resources.
//!
//! Resources are data that is shared globally across the App. Resources are stored in a World and can be accessed from any System. Resources are also automatically synchronized across threads when using Bevy's parallel systems.

use bevy::prelude::*;

#[derive(Resource)]
pub struct Score(pub u32);

impl Default for Score {
    fn default() -> Self {
        Score(0)
    }
}

#[derive(Resource)]
pub struct PlayerSpeed {
    points: usize,
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

impl PlayerSpeed {
    const DEFAULT_POINTS: usize = 3;
    const POINT_SPEEDS: &'static [f32] = &[250., 325., 400., 500.0, 600., 700., 900., 1100., 1300.];

    pub fn change_points(&mut self, delta_points: i32) {
        let points = (self.points as i32 + delta_points).max(0);
        self.points = (points as usize).min(Self::POINT_SPEEDS.len() - 1);
        println!(
            "PlayerSpeed.points: {}/{}",
            self.points,
            Self::POINT_SPEEDS.len() - 1
        );
    }

    pub fn get_speed(&self) -> f32 {
        Self::POINT_SPEEDS[self.points]
    }
}

impl Default for PlayerSpeed {
    fn default() -> Self {
        PlayerSpeed {
            points: Self::DEFAULT_POINTS,
        }
    }
}
