//! set up walls at the boundaries of the window that objects can collide with

use crate::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};

/// Resource to track the spawned wall entities
#[derive(Default, Resource)]
struct WallEntities {
    north: Option<Entity>,
    south: Option<Entity>,
    west: Option<Entity>,
    east: Option<Entity>,
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(WallEntities::default())
        .add_systems(Startup, setup_walls)
        .add_systems(Update, update_walls_on_resize);
}

/// Run on startup or when the window is resized
fn setup_walls(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    mut walls: ResMut<WallEntities>,
) {
    let Ok(window) = window.get_single() else {
        warn!("no primary window, not implementing bounded movement");
        return;
    };
    let width = window.resolution.width();
    let height = window.resolution.height();

    despawn_walls(&mut commands, &mut walls);

    *walls = WallEntities {
        south: Some(spawn_wall(
            &mut commands,
            width + 9.,
            10.0,
            0.0,
            -height / 2.0,
        )),
        north: Some(spawn_wall(
            &mut commands,
            width + 9.,
            10.0,
            0.0,
            height / 2.0,
        )),
        west: Some(spawn_wall(
            &mut commands,
            10.0,
            height + 9.,
            -width / 2.0,
            0.0,
        )),
        east: Some(spawn_wall(
            &mut commands,
            10.0,
            height + 9.,
            width / 2.0,
            0.0,
        )),
    };
}

/// Update the walls when the window is resized
fn update_walls_on_resize(
    mut commands: Commands,
    resize_event: EventReader<WindowResized>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut walls: ResMut<WallEntities>,
) {
    if resize_event.is_empty() {
        return;
    }

    let Ok(window) = window.get_single() else {
        warn!("No primary window, not updating bounded movement on resize");
        return;
    };

    let width = window.resolution.width();
    let height = window.resolution.height();

    despawn_walls(&mut commands, &mut walls);

    *walls = WallEntities {
        south: Some(spawn_wall(&mut commands, width, 1.0, 0.0, -height / 2.0)),
        north: Some(spawn_wall(&mut commands, width, 1.0, 0.0, height / 2.0)),
        west: Some(spawn_wall(&mut commands, 1.0, height, -width / 2.0, 0.0)),
        east: Some(spawn_wall(&mut commands, 1.0, height, width / 2.0, 0.0)),
    };
}

/// Despawn all existing walls
fn despawn_walls(commands: &mut Commands, walls: &mut WallEntities) {
    if let Some(entity) = walls.north.take() {
        commands.entity(entity).despawn();
    }
    if let Some(entity) = walls.south.take() {
        commands.entity(entity).despawn();
    }
    if let Some(entity) = walls.west.take() {
        commands.entity(entity).despawn();
    }
    if let Some(entity) = walls.east.take() {
        commands.entity(entity).despawn();
    }
}

/// Helper function to spawn a wall with the given size and position
fn spawn_wall(commands: &mut Commands, width: f32, height: f32, x: f32, y: f32) -> Entity {
    commands
        .spawn((
            Collider::cuboid(width, height),
            TransformBundle::from(Transform::from_xyz(x, y, 0.0)),
        ))
        .id()
}
