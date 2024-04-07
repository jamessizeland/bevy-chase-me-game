use super::{
    enemy::{Enemy, EnemyState},
    player::Player,
    state::InGameState,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::dynamics::Velocity;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Friction(0.9)).add_systems(
            Update,
            (
                bounded_movement,
                player_movement,
                chase_movement.after(player_movement),
            )
                .run_if(in_state(InGameState::Play)),
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

#[derive(Component, Default, Clone, Copy, Debug, Reflect)]
pub struct Momentum {
    pub max_speed: f32,
    pub mass: f32,
    pub thrust: f32,
}

impl Momentum {
    /// Create a new Momentum component with the given max_speed, mass, and thrust
    pub fn new(max_speed: f32, mass: f32, thrust: f32) -> Self {
        Self {
            max_speed,
            mass,
            thrust,
        }
    }
}

#[derive(Resource)]
pub struct Friction(pub f32);

#[derive(Component)]
pub struct KeyboardMovement;

/// This system changes the velocity vector of objects with Momentum based on player input. Pressing a move direction key increases the velocity in that direction based on the mass and thrust of the object. Releasing the key decreases the velocity in that direction based on the friction and mass of the object.
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut objects: Query<(&mut Transform, &Player, &Momentum, &mut Velocity)>,
    friction: Res<Friction>,
    time: Res<Time>,
) {
    for (mut transform, _, momentum, mut velocity) in &mut objects {
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
        velocity.linvel += acceleration * time.delta_seconds();
        velocity.linvel *= 1.0 - friction.0 * time.delta_seconds();
        velocity.linvel = velocity.linvel.clamp(
            Vec2::splat(-momentum.max_speed),
            Vec2::splat(momentum.max_speed),
        );
        transform.translation += Vec3::new(velocity.linvel.x, velocity.linvel.y, 0.0);
        // rotate the object to face the direction of movement assuming that the object is facing up to begin with
        let angle = velocity.linvel.angle_between(Vec2::Y);
        transform.rotation = Quat::from_rotation_z(-angle);
    }
}

/// This system changes the velocity vector of objects with Chaser based on the position of the target object. The object will move towards the closest target object at a speed determined by the mass and thrust of the object.
pub fn chase_movement(
    mut commands: Commands,
    mut objects: Query<(Entity, &Transform, &Enemy, &Momentum, &mut Velocity)>,
    target: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    for (entity, transform, enemy, momentum, mut velocity) in objects.iter_mut() {
        match enemy.state {
            EnemyState::Moving => {
                // find the closest target
                let target_transform = target.iter().fold(None, |closest, target| {
                    let distance = transform.translation.distance(target.translation);
                    closest.map_or(Some((distance, target)), |(closest_distance, _)| {
                        if distance < closest_distance {
                            Some((distance, target))
                        } else {
                            closest
                        }
                    })
                });
                // accelerate towards the target
                let Some((_dist, target_transform)) = target_transform else {
                    warn!("no target found for chaser movement");
                    return;
                };
                let direction =
                    target_transform.translation.truncate() - transform.translation.truncate();
                let acceleration = direction.normalize() * momentum.thrust / momentum.mass;

                velocity.linvel += acceleration * time.delta_seconds();
                velocity.linvel = velocity.linvel.clamp(
                    Vec2::splat(-momentum.max_speed),
                    Vec2::splat(momentum.max_speed),
                );
                let new_translation =
                    Vec3::new(velocity.linvel.x, velocity.linvel.y, 0.0) + transform.translation;
                // rotate the object to face the direction of movement assuming that the object is facing up to begin with
                let angle = velocity.linvel.angle_between(Vec2::Y);
                let new_rotation = Quat::from_rotation_z(-angle);
                commands.entity(entity).insert(Transform {
                    translation: new_translation,
                    rotation: new_rotation,
                    ..*transform
                });
            }
            EnemyState::Stopped => {
                velocity.linvel = Vec2::ZERO;
            }
        }
    }
}
