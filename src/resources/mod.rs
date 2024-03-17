//! # Resources
//! Entities and Components are great for representing complex, query-able groups of data. But most Apps will also require "globally unique" data of some kind. In Bevy ECS, we represent globally unique data using Resources.
//!
//! Resources are data that is shared globally across the App. Resources are stored in a World and can be accessed from any System. Resources are also automatically synchronized across threads when using Bevy's parallel systems.

use bevy::prelude::*;

#[derive(Resource)]
pub struct Money(pub f32);

impl Default for Money {
    fn default() -> Self {
        Money(100.0)
    }
}
