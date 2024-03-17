use bevy::{input::ButtonInput, prelude::*};

use crate::{components, resources};
use rand::Rng;

/// Spawn a pig when the player presses the space bar, and deduct the cost from the player's money
pub fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut money: ResMut<resources::Money>,
    player: Query<&Transform, With<components::Player>>,
) {
    let pig_cost = 10.0;
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    let player_transform = player.single();

    if money.0 < pig_cost {
        return;
    }
    money.0 -= pig_cost;
    info!(
        "Spent ${} on a pig, remaining money: ${}",
        pig_cost, money.0
    );
    let texture = asset_server.load("pig.png");
    commands.spawn((
        SpriteBundle {
            texture,
            transform: *player_transform,
            ..default()
        },
        components::Pig {
            lifetime: Timer::from_seconds(5.0, TimerMode::Once),
            move_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            stop_timer: Timer::from_seconds(0.5, TimerMode::Once),
            ..Default::default()
        },
    ));
}

/// Remove pigs from the map when their lifetime is up, and give the player money
pub fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut components::Pig)>,
    mut money: ResMut<resources::Money>,
) {
    for (entity, mut pig) in pigs.iter_mut() {
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished() {
            money.0 += 15.0;
            commands.entity(entity).despawn();
            info!("Pig sold, gained $15, remaining money: ${}", money.0);
        }
    }
}

/// Let pigs randomly walk about the map, stopping and starting movement at random intervals
pub fn pig_movement(mut pigs: Query<(&mut Transform, &mut components::Pig)>, time: Res<Time>) {
    for (mut transform, mut pig) in pigs.iter_mut() {
        if pig.is_moving {
            let movement_amount = 50.0 * time.delta_seconds();
            let mut translation = Vec3::ZERO;
            if pig.direction.x == 0.0 && pig.direction.y == 0.0 {
                pig.is_moving = false;
                pig.stop_timer.reset();
            } else {
                translation += pig.direction * movement_amount;
                transform.translation += translation;
                pig.stop_timer.tick(time.delta());
                if pig.stop_timer.finished() {
                    pig.direction = random_direction();
                    pig.is_moving = false;
                    pig.move_timer.reset();
                }
            }
        } else {
            pig.move_timer.tick(time.delta());
            if pig.move_timer.finished() {
                pig.direction = random_direction();
                pig.is_moving = true;
                pig.stop_timer.reset();
            }
        }
    }
}

fn random_direction() -> Vec3 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-1.0..=1.0);
    let y = rng.gen_range(-1.0..=1.0);
    Vec3::new(x, y, 0.0).normalize()
}
