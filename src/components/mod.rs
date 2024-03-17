//! # Components
//! Components are the building blocks of Entities. They are used to add data to an Entity. For example, a Position component might represent the position of an Entity in 3D space. A Health component might represent the health of an Entity. Components are plain old data (POD) and are generally very simple. Components are used to add data to an Entity and are queried using Queries.

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct Pig {
    pub lifetime: Timer,
    pub is_moving: bool,
    pub direction: Vec3,
    pub stop_timer: Timer,
    pub move_timer: Timer,
}

impl Default for Pig {
    fn default() -> Self {
        Self {
            lifetime: Default::default(),
            is_moving: Default::default(),
            direction: Default::default(),
            stop_timer: Default::default(),
            move_timer: Default::default(),
        }
    }
}
