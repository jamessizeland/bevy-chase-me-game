use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

// set up walls at the boundaries of the window that objects can collide with

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_walls);
    }
}

fn setup_walls(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let Ok(window) = window.get_single() else {
        warn!("no primary window, not implementing bounded movement");
        return;
    };
    let width = window.resolution.width();
    let height = window.resolution.height();

    // South wall
    commands.spawn((
        Collider::cuboid(width, 1.0),
        TransformBundle::from(Transform::from_xyz(width / 2.0, 0.0, 0.0)),
    ));
    // North wall
    commands.spawn((
        Collider::cuboid(width, 1.0),
        TransformBundle::from(Transform::from_xyz(width / 2.0, height, 0.0)),
    ));
    // West wall
    commands.spawn((
        Collider::cuboid(1.0, height),
        TransformBundle::from(Transform::from_xyz(0.0, height / 2.0, 0.0)),
    ));
    // East wall
    commands.spawn((
        Collider::cuboid(1.0, height),
        TransformBundle::from(Transform::from_xyz(width, height / 2.0, 0.0)),
    ));
}
