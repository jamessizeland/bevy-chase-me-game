use super::{resources::EnemyStrengthRange, Enemy, EnemyParent, EnemyState, GameTime, Score};
use crate::game::{
    events::{ShipDestroyed, ShipHit},
    movement::Momentum,
    player::Player,
};
use bevy::{prelude::*, utils::hashbrown::Equivalent, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

/// Spawn a enemy at a random location on the map, after a random interval
pub fn spawn_enemy(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<EnemyParent>>,
    game_time: ResMut<GameTime>,
    max_enemy_strength: Res<EnemyStrengthRange>,
    time: Res<Time>,
    score: Res<Score>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    // only spawn enemies every 5 seconds
    if !(game_time.time % 5.0 < time.delta_seconds()) {
        return;
    }

    let parent = parent.single();
    let player_transform = player.single();
    let Ok(window) = window.get_single() else {
        warn!("no primary window, not spawning enemy");
        return;
    };
    let half_width = window.resolution.width() / 2.0;
    let half_height = window.resolution.height() / 2.0;

    // spawn enemies with different shapes, lifetimes, max speeds, max energies, recharge rates, masses and thrusts.  These are randomized but get more difficult as the game progresses.
    // The colour and shape of the enemy should be associated with the difficulty level of the spawned enemy's stats.
    // The enemy should move towards the player, and stop when it runs out of energy.  It should then recharge its energy and start moving again.
    // The enemy should despawn after a certain amount of time, and the player should get points for surviving.
    // The player should lose health if they collide with an enemy.
    let bias = (score.0 / 100.0).clamp(0.5, 2.0); // the bias is the percentage of the max power of the enemy that the new enemy should have
    let (momentum, mut enemy) = max_enemy_strength.get_enemy_stats(bias);

    let radius = max_enemy_strength.get_radius(momentum.mass); // radius of the enemy

    assert!(
        radius < half_width && radius < half_height,
        "Enemy radius is too large"
    );
    // spawn location, must be at least 20% of the window width away from the player and at least its own radius away from a wall.
    let mut rng = rand::thread_rng();
    let (random_x, random_y) = loop {
        let random_x = rng.gen_range(-half_width + radius..=half_width - radius);
        let random_y = rng.gen_range(-half_height + radius..=half_height - radius);
        if random_x < player_transform.translation.x - 0.4 * half_width
            || random_x > player_transform.translation.x + 0.4 * half_width
            || random_y < player_transform.translation.y - 0.4 * half_height
            || random_y > player_transform.translation.y + 0.4 * half_height
        {
            break (random_x, random_y);
        }
    };
    // info!("random_x: {}, random_y: {}", random_x, random_y);

    // generate a colour and shape based on the difficulty level of the enemy, which is based on the enemy's stats. Red should be the easiest, scaling up to blue the hardest.
    let colour = max_enemy_strength.get_colour(&momentum, &enemy);
    enemy.colour = colour; // set to use for explosion colour on death
    let path = max_enemy_strength.get_shape(&momentum, &enemy);

    // info!("Spawning enemy with stats: {:?}", (&momentum, &enemy));

    commands
        .spawn((
            ShapeBundle {
                path,
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    ..default()
                },
                ..default()
            },
            Fill::color(colour),
            Stroke::new(Color::BLACK, 1.0),
            Name::new(format!("Enemy {}", rand::random::<u16>())),
            momentum,
            Collider::ball(radius),
            ColliderMassProperties::Density(0.2),
            Restitution::new(0.9),
            RigidBody::Dynamic,
            Ccd::enabled(),
            GravityScale(0.0),
            Velocity {
                linvel: Vec2::new(0.1, 0.1),
                angvel: 2.0,
            },
            ActiveEvents::COLLISION_EVENTS,
            enemy,
        ))
        .set_parent(parent);
}

/// Remove enemies from the map when their lifetime is up, and give the player score
pub fn enemy_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut enemies: Query<(&Transform, Entity, &mut Enemy, &Momentum)>,
    mut score: ResMut<Score>,
    mut ship_destroyed_events: EventWriter<ShipDestroyed>,
) {
    let time_passed = time.delta().as_secs_f32();
    for (transform, entity, mut enemy, momentum) in enemies.iter_mut() {
        match enemy.state {
            EnemyState::Stopped => {
                enemy.energy += (enemy.recharge_rate * time_passed).clamp(0.0, enemy.max_energy);
                if enemy.energy >= enemy.max_energy {
                    enemy.state = EnemyState::Moving;
                    return;
                }
            }
            EnemyState::Moving => {
                enemy.energy = (enemy.energy - time_passed).clamp(0.0, enemy.max_energy);
                enemy.lifetime.tick(time.delta());
                if enemy.energy <= 0.0 {
                    enemy.state = EnemyState::Stopped;
                    return;
                };
                if enemy.lifetime.finished() {
                    score.0 += calc_strength(&momentum, &enemy);
                    commands.entity(entity).remove_parent().despawn();
                    info!("Survied! Adding score");
                    ship_destroyed_events.send(ShipDestroyed {
                        x: transform.translation.x,
                        y: transform.translation.y,
                        colour: enemy.colour,
                    });
                }
            }
        }
    }
}

/// Enemies lose energy when they get hit
pub fn enemy_hit(
    mut objects: Query<(Entity, &mut Enemy, &Velocity)>,
    mut hit_events: EventReader<ShipHit>,
) {
    for events in hit_events.read() {
        for (entity, mut enemy, velocity) in objects.iter_mut() {
            // read events and reduce energy of enemies.
            if events.id.equivalent(&entity) {
                info!(
                    "enemy {} hit something at {} with {}/{} energy",
                    events.id, velocity.linvel, enemy.energy, enemy.max_energy
                );
                enemy.energy -= enemy.max_energy * 0.2; // drop energy by 20% on collision
            }
        }
    }
}

/// Calculate the strength of the enemy based on its momentum and energy
pub fn calc_strength(momentum: &Momentum, enemy: &Enemy) -> f32 {
    momentum.max_speed * momentum.thrust / momentum.mass + enemy.max_energy + enemy.recharge_rate
}

/// Generate a random number between min and (max * bias)
pub fn randomize(range: (f32, f32), bias: f32) -> f32 {
    let (min, max) = range;
    assert!(min > 0.0, "min must be greater than 0");
    assert!(bias > 0.0, "bias must be greater than 0");
    let max = (max * bias).clamp(min, f32::INFINITY);
    assert!(max > 0.0, "max must be greater than 0");
    assert!(min <= max, "min must be less than or equal to max * bias");
    rand::thread_rng().gen_range(min..=max)
}
