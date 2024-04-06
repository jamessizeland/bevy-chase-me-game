use super::state::InGameState;
use bevy::{prelude::*, window::PrimaryWindow};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Friction(0.9)).add_systems(
            Update,
            (bounded_movement, keyboard_movement).run_if(in_state(InGameState::Play)),
        );
    }
}

#[derive(Component)]
pub struct BoundedMovement;

pub fn bounded_movement(
    mut objects: Query<(&mut Transform, &BoundedMovement)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.get_single() else {
        warn!("no primary window, not implementing bounded movement");
        return;
    };
    let width = window.resolution.width();
    let height = window.resolution.height();
    for (mut transform, _) in &mut objects {
        transform.translation.x = transform.translation.x.clamp(0.0, width);
        transform.translation.y = transform.translation.y.clamp(0.0, height);
    }
}

#[derive(Component, Default, Clone, Copy)]
pub struct Momentum {
    pub velocity: Vec2,
    pub max_speed: f32,
    pub mass: f32,
    pub thrust: f32,
}

#[derive(Resource)]
pub struct Friction(pub f32);

impl Momentum {
    pub fn new(max_speed: f32, mass: f32, thrust: f32) -> Self {
        Self {
            velocity: Vec2::ZERO,
            max_speed,
            mass,
            thrust,
        }
    }
}

#[derive(Component)]
pub struct KeyboardMovement;

/// This system changes the velocity vector of objects with Momentum based on keyboard input. Pressing a move direction key increases the velocity in that direction based on the mass and thrust of the object. Releasing the key decreases the velocity in that direction based on the friction and mass of the object.
pub fn keyboard_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut objects: Query<(&mut Transform, &KeyboardMovement, &mut Momentum)>,
    friction: Res<Friction>,
    time: Res<Time>,
) {
    for (mut transform, _, mut momentum) in &mut objects {
        let mut acceleration = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            acceleration.y += momentum.thrust / momentum.mass;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            acceleration.y -= momentum.thrust / momentum.mass;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            acceleration.x -= momentum.thrust / momentum.mass;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            acceleration.x += momentum.thrust / momentum.mass;
        }
        momentum.velocity += acceleration * time.delta_seconds();
        momentum.velocity *= 1.0 - friction.0 * time.delta_seconds();
        momentum.velocity = momentum.velocity.clamp(
            Vec2::splat(-momentum.max_speed),
            Vec2::splat(momentum.max_speed),
        );
        transform.translation += Vec3::new(momentum.velocity.x, momentum.velocity.y, 0.0);
        // rotate the object to face the direction of movement assuming that the object is facing up to begin with
        let angle = momentum.velocity.angle_between(Vec2::Y);
        transform.rotation = Quat::from_rotation_z(-angle);
    }
}
