use super::{movement::Momentum, player::Player, state::InGameState, GameTime, Score};
use crate::AppState;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

/// A enemy parent, which spawns enemies
#[derive(Component)]
pub struct EnemyParent;

/// Spawn a enemy parent when the game starts
fn spawn_enemy_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        EnemyParent,
        Name::new("Enemy Parent"),
    ));
}

fn despawn_all_enemies(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component, Default, Reflect, Debug, PartialEq)]
pub enum EnemyState {
    #[default]
    Stopped,
    Moving,
}

/// Plugin to manage enemies
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MaxEnemyStrength>()
            .add_systems(Startup, spawn_enemy_parent)
            .add_systems(Update, (spawn_enemy).run_if(in_state(InGameState::Play)))
            .add_systems(
                PostUpdate,
                (enemy_lifetime).run_if(in_state(InGameState::Play)),
            )
            .add_systems(OnExit(AppState::InGame), despawn_all_enemies)
            .register_type::<Enemy>(); // used for debug inspection
    }
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct Enemy {
    /// The lifetime of the enemy
    pub lifetime: Timer,
    /// The current energy of the enemy, before it needs to stop and recharge
    pub energy: f32,
    /// The maximum energy of the enemy
    pub max_energy: f32,
    /// rate at which the enemy recharges energy
    pub recharge_rate: f32,
    /// The state of the enemy
    pub state: EnemyState,
}

impl Enemy {
    /// Create a new enemy
    pub fn new(lifetime: f32, max_energy: f32, recharge_rate: f32) -> Self {
        Self {
            lifetime: Timer::from_seconds(lifetime, TimerMode::Once),
            // start with a random amount of energy
            energy: rand::thread_rng().gen_range(0.0..=max_energy),
            max_energy,
            recharge_rate,
            state: EnemyState::Stopped,
        }
    }
}

/// Spawn a enemy at a random location on the map, after a random interval
fn spawn_enemy(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<EnemyParent>>,
    game_time: ResMut<GameTime>,
    max_enemy_strength: Res<MaxEnemyStrength>,
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
        warn!("no primary window, not implementing bounded movement");
        return;
    };
    let width = window.resolution.width();
    let height = window.resolution.height();

    // spawn enemies with different shapes, lifetimes, max speeds, max energies, recharge rates, masses and thrusts.  These are randomized but get more difficult as the game progresses.
    // The colour and shape of the enemy should be associated with the difficulty level of the spawned enemy's stats.
    // The enemy should move towards the player, and stop when it runs out of energy.  It should then recharge its energy and start moving again.
    // The enemy should despawn after a certain amount of time, and the player should get points for surviving.
    // The player should lose health if they collide with an enemy.
    let bias = (score.0 / 100.0).clamp(0.5, 2.0); // the bias is the percentage of the max power of the enemy that the new enemy should have
    let (momentum, enemy) = max_enemy_strength.get_enemy_stats(bias);

    let radius = max_enemy_strength
        .get_radius(momentum.mass)
        .clamp(2.0, height / 2.0 - 1.0); // radius of the enemy

    assert!(
        radius < width / 2.0 && radius < height / 2.0,
        "Enemy radius is too large"
    );
    // spawn location, must be at least 20% of the window width away from the player and at least its own radius away from a wall.
    let mut rng = rand::thread_rng();
    let (random_x, random_y) = loop {
        let random_x = rng.gen_range(0.0 + radius..=width - radius);
        let random_y = rng.gen_range(0.0 + radius..=height - radius);
        if random_x < player_transform.translation.x - 0.2 * width
            || random_x > player_transform.translation.x + 0.2 * width
            || random_y < player_transform.translation.y - 0.2 * height
            || random_y > player_transform.translation.y + 0.2 * height
        {
            break (random_x, random_y);
        }
    };
    info!("random_x: {}, random_y: {}", random_x, random_y);

    // generate a colour and shape based on the difficulty level of the enemy, which is based on the enemy's stats. Red should be the easiest, scaling up to blue the hardest.
    let colour = max_enemy_strength.get_colour(&momentum, &enemy);
    let path = max_enemy_strength.get_shape(&momentum, &enemy);

    info!("Spawning enemy with stats: {:?}", (&momentum, &enemy));

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
            Velocity::default(),
            enemy,
        ))
        .set_parent(parent);
}

/// Remove enemies from the map when their lifetime is up, and give the player score
fn enemy_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut enemies: Query<(Entity, &mut Enemy, &Momentum)>,
    mut score: ResMut<Score>,
) {
    let time_passed = time.delta().as_secs_f32();
    for (entity, mut enemy, momentum) in enemies.iter_mut() {
        match enemy.state {
            EnemyState::Stopped => {
                enemy.energy += (enemy.recharge_rate * time_passed).clamp(0.0, enemy.max_energy);
                if enemy.energy >= enemy.max_energy {
                    enemy.state = EnemyState::Moving;
                    return;
                }
            }
            EnemyState::Moving => {
                enemy.energy -= time_passed.clamp(0.0, enemy.energy);
                enemy.lifetime.tick(time.delta());
                if enemy.energy <= 0.0 {
                    enemy.state = EnemyState::Stopped;
                    return;
                };
                if enemy.lifetime.finished() {
                    score.0 += calc_strength(&momentum, &enemy);
                    commands.entity(entity).remove_parent().despawn();
                    info!("Survied! Adding score");
                }
            }
        }
    }
}

/// Calculate the strength of the enemy based on its momentum and energy
fn calc_strength(momentum: &Momentum, enemy: &Enemy) -> f32 {
    momentum.max_speed * momentum.thrust / momentum.mass + enemy.max_energy + enemy.recharge_rate
}

/// Generate a random number between 0 and max
fn randomize(max: f32) -> f32 {
    assert!(max > 0.0, "max must be greater than 0");
    rand::thread_rng().gen_range(0.0..=max)
}

/// The max power of an enemy
#[derive(Resource)]
pub struct MaxEnemyStrength {
    max_max_speed: f32,
    max_max_energy: f32,
    max_recharge_rate: f32,
    max_mass: f32,
    max_thrust: f32,
    max_lifetime: f32,
    max_radius: f32,
}

impl Default for MaxEnemyStrength {
    fn default() -> Self {
        MaxEnemyStrength {
            max_max_speed: 12.0,
            max_max_energy: 120.0,
            max_recharge_rate: 60.0,
            max_mass: 5.0,
            max_thrust: 20.0,
            max_lifetime: 120.0,
            max_radius: 5.0,
        }
    }
}

impl MaxEnemyStrength {
    /// Get the max power of the enemy
    fn get_power(&self) -> f32 {
        calc_strength(
            &Momentum::new(self.max_max_speed, self.max_mass, self.max_thrust),
            &Enemy::new(
                self.max_lifetime,
                self.max_max_energy,
                self.max_recharge_rate,
            ),
        )
    }
    /// Generate stats for a new enemy, based on the max power of the enemy and a bias that reduces each stat by a percentage
    fn get_enemy_stats(&self, bias: f32) -> (Momentum, Enemy) {
        let momentum = Momentum::new(
            randomize(self.max_max_speed) * bias,
            randomize(self.max_mass),
            randomize(self.max_thrust) * bias,
        );
        let enemy = Enemy::new(
            (randomize(self.max_lifetime) * bias).clamp(10.0, self.max_lifetime),
            randomize(self.max_max_energy) * bias,
            randomize(self.max_recharge_rate) * bias,
        );
        (momentum, enemy)
    }
    /// Generate the radius of the new enemy based on the max radius of the enemy, max mass and its current mass
    fn get_radius(&self, mass: f32) -> f32 {
        let mass_ratio = self.max_mass / mass;
        self.max_radius * mass_ratio
    }
    /// Get the colour of the enemy based on its momentum and energy
    fn get_colour(&self, momentum: &Momentum, enemy: &Enemy) -> Color {
        let difficult = calc_strength(momentum, enemy);
        let max_difficulty = self.get_power();
        let relative_difficulty = difficult / max_difficulty;
        // have 10 difficulty colours
        if relative_difficulty < 0.1 {
            Color::RED
        } else if relative_difficulty < 0.2 {
            Color::rgb(1.0, 0.5, 0.0)
        } else if relative_difficulty < 0.3 {
            Color::rgb(1.0, 1.0, 0.0)
        } else if relative_difficulty < 0.4 {
            Color::rgb(0.5, 1.0, 0.0)
        } else if relative_difficulty < 0.5 {
            Color::rgb(0.0, 1.0, 0.0)
        } else if relative_difficulty < 0.6 {
            Color::rgb(0.0, 1.0, 0.5)
        } else if relative_difficulty < 0.7 {
            Color::rgb(0.0, 1.0, 1.0)
        } else if relative_difficulty < 0.8 {
            Color::rgb(0.0, 0.5, 1.0)
        } else if relative_difficulty < 0.9 {
            Color::rgb(0.0, 0.0, 1.0)
        } else {
            Color::BLUE
        }
    }
    /// Get the shape of the enemy based on its momentum and energy
    fn get_shape(&self, momentum: &Momentum, enemy: &Enemy) -> Path {
        let difficulty = calc_strength(momentum, enemy);
        let max_difficulty = self.get_power();
        let radius = self.get_radius(momentum.mass);
        let relative_difficulty = difficulty / max_difficulty;
        // if relative_difficulty < 0.33 {
        //     GeometryBuilder::build_as(&shapes::Circle {
        //         radius,
        //         center: Vec2::ZERO,
        //     })
        // } else if relative_difficulty < 0.66 {
        //     GeometryBuilder::build_as(&shapes::Polygon {
        //         points: vec![
        //             Vec2::new(-radius, -radius),
        //             Vec2::new(radius, -radius),
        //             Vec2::new(0.0, radius),
        //         ],
        //         closed: true,
        //     })
        // } else {
        //     GeometryBuilder::build_as(&shapes::Polygon {
        //         points: vec![
        //             Vec2::new(-radius, -radius),
        //             Vec2::new(radius, -radius),
        //             Vec2::new(0.0, radius),
        //             Vec2::new(-radius, radius),
        //         ],
        //         closed: true,
        //     })
        // }
        GeometryBuilder::build_as(&shapes::Circle {
            radius,
            center: Vec2::ZERO,
        })
    }
}
