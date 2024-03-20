use super::player::Player;
use crate::resources;
use bevy::{input::ButtonInput, prelude::*};
use rand::Rng;

/// A pig parent, which spawns pigs
#[derive(Component)]
pub struct PigParent;

/// Spawn a pig parent when the game starts
pub fn spawn_pig_parent(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), PigParent, Name::new("Pig Parent")));
}

/// Plugin to manage pigs
pub struct PigPlugin;

impl Plugin for PigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pig_parent)
            .add_systems(Update, (spawn_pig, pig_lifetime, pig_movement))
            .register_type::<Pig>(); // used for debug inspection
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Pig {
    pub lifetime: Timer,
    pub is_moving: bool,
    pub direction: Vec3,
    pub stop_timer: Timer,
    pub move_timer: Timer,
}

/// Spawn a pig when the player presses the space bar, and deduct the cost from the player's money
pub fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut money: ResMut<resources::Money>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<PigParent>>,
) {
    let pig_cost = 10.0;
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    let player_transform = player.single();
    let parent = parent.single();

    if money.0 < pig_cost {
        return;
    }
    money.0 -= pig_cost;
    info!(
        "Spent ${} on a pig, remaining money: ${}",
        pig_cost, money.0
    );
    let texture = asset_server.load("pig.png");

    // commands.entity(parent).with_children(|commands| {
    commands
        .spawn((
            SpriteBundle {
                texture,
                transform: *player_transform,
                ..default()
            },
            Pig {
                lifetime: Timer::from_seconds(5.0, TimerMode::Once),
                move_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                stop_timer: Timer::from_seconds(0.5, TimerMode::Once),
                ..Default::default()
            },
            Name::new(format!("Pig {}", rand::random::<u16>())),
        ))
        .set_parent(parent);
    // });
}

/// Remove pigs from the map when their lifetime is up, and give the player money
pub fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig)>,
    mut money: ResMut<resources::Money>,
) {
    for (entity, mut pig) in pigs.iter_mut() {
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished() {
            money.0 += 15.0;
            commands.entity(entity).remove_parent().despawn();
            info!("Pig sold, gained $15, remaining money: ${}", money.0);
        }
    }
}

/// Let pigs randomly walk about the map, stopping and starting movement at random intervals
pub fn pig_movement(mut pigs: Query<(&mut Transform, &mut Pig)>, time: Res<Time>) {
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
