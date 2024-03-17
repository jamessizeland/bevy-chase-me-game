//! # Systems
//! Systems are pieces of functionality, which are run by Bevy. They are typically implemented using regular Rust functions.
//!
//! This is how you implement all your game logic. Each system specifies what data it needs to access to do its thing, and Bevy will run them in parallel when possible.
//!
//! These functions can only take [special parameter types](https://bevy-cheatbook.github.io/builtins.html#systemparams), to specify what data you need access to. If you use unsupported parameter types in your function, you will get confusing compiler errors!
//!
//! Some of the possibilities are:
//!
//! - accessing resources using Res/ResMut
//! - accessing components of entities using queries (Query)
//! - creating/destroying entities, components, and resources using Commands (Commands)
//! - sending/receiving events using EventWriter/EventReader

use bevy::{prelude::*, render::camera::ScalingMode};

use crate::plugins::player::Player;

/// This system runs once at the start of the app
pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };
    commands.spawn(camera);

    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player { speed: 100.0 },
    ));
}
