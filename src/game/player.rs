//! Plugin handling the player character in particular.
//! Note that this is separate from the `movement` module as that could be used
//! for other characters as well.

use super::movement::{KeyboardMovement, Momentum};
use crate::prelude::*;
use bevy::{
    color::palettes::css::SILVER,
    ecs::{system::RunSystemOnce as _, world::Command},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>(); // used for debug inspection
    app.add_systems(OnEnter(InGameState::Preparation), despawn_player);
}

#[derive(Component, Default, Reflect)]
pub struct Player;

#[derive(Debug)]
pub struct SpawnPlayer {
    pub max_speed: f32,
    pub mass: f32,
    pub thrust: f32,
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_player);
    }
}

/// Spawns the player character in the center of the window.
fn spawn_player(
    In(SpawnPlayer {
        max_speed,
        mass,
        thrust,
    }): In<SpawnPlayer>, // required to run this system once
    mut commands: Commands,
) {
    let start_pos = Vec3::new(0.0, 0.0, 0.0);
    commands.spawn((
        ShapeBundle {
            path: player_shape(),
            spatial: SpatialBundle::from_transform(Transform::from_translation(start_pos)),
            ..default()
        },
        Fill::color(SILVER),
        Stroke::new(Color::BLACK, 1.0),
        Name::new("Player"),
        Momentum::new(max_speed, mass, thrust),
        KeyboardMovement,
        Collider::ball(7.5),
        ColliderMassProperties::Density(0.6),
        Restitution::new(0.9),
        RigidBody::Dynamic,
        Ccd::enabled(),
        GravityScale(0.0),
        Velocity {
            linvel: Vec2::new(0.0, max_speed * 0.1),
            angvel: 0.0,
        },
        Player,
        ActiveEvents::all(),
    ));
}

/// Despawn the player character when the game ends.
fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// The shape of the player character. This is a simple bird shape.
fn player_shape() -> Path {
    let points = vec![
        Vec2::new(0.0, 15.0),   // tip of the beak
        Vec2::new(-5.0, 5.0),   // start of left wing
        Vec2::new(-15.0, -5.0), // end of left wing
        Vec2::new(-5.0, -2.5),  // left side of the tail
        Vec2::new(0.0, -10.0),  // tip of the tail
        Vec2::new(5.0, -2.5),   // right side of the tail
        Vec2::new(15.0, -5.0),  // end of right wing
        Vec2::new(5.0, 5.0),    // start of right wing
    ];
    GeometryBuilder::build_as(&shapes::Polygon {
        points,
        closed: true,
    })
}
