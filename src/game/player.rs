use super::movement::{KeyboardMovement, Momentum};
use crate::{
    window::{WINDOW_USABLE_WORLD_WIDTH, WINDOW_WORLD_HEIGHT},
    AppState,
};
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player)
            .add_systems(OnExit(AppState::InGame), despawn_player)
            .register_type::<Player>(); // used for debug inspection
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    let start_pos = Vec3::new(
        WINDOW_USABLE_WORLD_WIDTH / 2.0,
        WINDOW_WORLD_HEIGHT / 2.0,
        0.0,
    );
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
    let triangle = shapes::Polygon {
        points,
        closed: true,
    };
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&triangle),
            spatial: SpatialBundle {
                transform: Transform {
                    translation: start_pos,
                    scale: Vec3::new(2.0, 2.0, 1.0),
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            },
            ..default()
        },
        Fill::color(Color::SILVER),
        Stroke::new(Color::BLACK, 1.0),
        Name::new("Player"),
        Momentum::new(5.0, 1.3, 10.0),
        KeyboardMovement,
        Collider::circle(7.5),
        Restitution::new(0.9),
        RigidBody::Dynamic,
        Player,
    ));
}

fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
