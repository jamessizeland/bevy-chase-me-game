use crate::{
    window::{WINDOW_USABLE_WORLD_WIDTH, WINDOW_WORLD_HEIGHT},
    AppState,
};

use super::{player::Player, state::InGameState, GameTime, Score};
use bevy::prelude::*;
use rand::Rng;

/// A pig parent, which spawns pigs
#[derive(Component)]
pub struct PigParent;

/// Spawn a pig parent when the game starts
fn spawn_pig_parent(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), PigParent, Name::new("Pig Parent")));
}

fn despawn_all_pigs(mut commands: Commands, query: Query<Entity, With<Pig>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Plugin to manage pigs
pub struct PigPlugin;

impl Plugin for PigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pig_parent)
            .add_systems(
                Update,
                (spawn_pig, pig_lifetime, pig_movement).run_if(in_state(InGameState::Play)),
            )
            .add_systems(OnExit(AppState::InGame), despawn_all_pigs)
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

/// Spawn a pig at a random location on the map, after a random interval
fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<PigParent>>,
    game_time: ResMut<GameTime>,
    time: Res<Time>,
) {
    let parent = parent.single();
    let player_transform = player.single();

    let texture = asset_server.load("sprites/pig.png");

    // spawn location, must be at least 20% of the window width away from the player
    let (random_x, random_y) = loop {
        let mut rng = rand::thread_rng();
        let random_x = rng.gen_range(0.0..=WINDOW_USABLE_WORLD_WIDTH as f32);
        let random_y = rng.gen_range(0.0..=WINDOW_WORLD_HEIGHT as f32);
        if random_x < player_transform.translation.x - 0.2 * WINDOW_USABLE_WORLD_WIDTH
            || random_x > player_transform.translation.x + 0.2 * WINDOW_USABLE_WORLD_WIDTH
            || random_y < player_transform.translation.y - 0.2 * WINDOW_WORLD_HEIGHT
            || random_y > player_transform.translation.y + 0.2 * WINDOW_WORLD_HEIGHT
        {
            break (random_x, random_y);
        }
    };

    // spawn pigs every 5 seconds
    if game_time.time % 5.0 < time.delta_seconds() {
        info!("Spawning pig");
    } else {
        return;
    }

    commands
        .spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_translation(Vec3::new(random_x, random_y, 0.0)),
                ..default()
            },
            Pig {
                lifetime: Timer::from_seconds(30.0, TimerMode::Once),
                move_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                stop_timer: Timer::from_seconds(0.5, TimerMode::Once),
                direction: random_direction(),
                ..Default::default()
            },
            Name::new(format!("Pig {}", rand::random::<u16>())),
        ))
        .set_parent(parent);
}

/// Remove pigs from the map when their lifetime is up, and give the player score
fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig)>,
    mut score: ResMut<Score>,
) {
    for (entity, mut pig) in pigs.iter_mut() {
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished() {
            score.0 += 15;
            commands.entity(entity).remove_parent().despawn();
            info!("Survied! Adding score");
        }
    }
}

/// Let pigs randomly walk about the map, stopping and starting movement at random intervals
fn pig_movement(mut pigs: Query<(&mut Transform, &mut Pig)>, time: Res<Time>) {
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

// /// pigs must move towards the player
// fn pig_movement(
//     mut pigs: Query<(&mut Transform, &mut Pig)>,
//     player: Query<&Transform, With<Player>>,
//     time: Res<Time>,
// ) {
//     let player_transform = player.single();
//     for (mut transform, _) in pigs.iter_mut() {
//         let movement_amount = 50.0 * time.delta_seconds();
//         let mut translation = Vec3::ZERO;
//         let direction = player_transform.translation - transform.translation;
//         let direction = direction.normalize();
//         translation += direction * movement_amount;
//         transform.translation += translation;
//     }
// }

fn random_direction() -> Vec3 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-1.0..=1.0);
    let y = rng.gen_range(-1.0..=1.0);
    Vec3::new(x, y, 0.0).normalize()
}
