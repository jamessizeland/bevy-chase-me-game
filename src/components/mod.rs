//! # Components
//! Components are the building blocks of Entities. They are used to add data to an Entity. For example, a Position component might represent the position of an Entity in 3D space. A Health component might represent the health of an Entity. Components are plain old data (POD) and are generally very simple. Components are used to add data to an Entity and are queried using Queries.

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}
