//! Module for the enemy entities and systems

mod resources;
mod systems;

use super::{
    events::{ShipDestroyed, ShipHit},
    state::InGameState,
    GameTime, Score,
};
use bevy::prelude::*;
use rand::Rng;
use resources::EnemyStrengthRange;
use systems::{enemy_hit, enemy_lifetime, spawn_enemy};

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

/// Despawn all enemies when the game ends
fn despawn_all_enemies(query: Query<Entity, With<Enemy>>, mut commands: Commands) {
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

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<EnemyStrengthRange>()
        .add_event::<ShipDestroyed>()
        .add_event::<ShipHit>()
        .add_systems(Startup, spawn_enemy_parent)
        .add_systems(
            Update,
            (spawn_enemy, enemy_hit, enemy_lifetime)
                .chain()
                .run_if(in_state(InGameState::Playing)),
        )
        .add_systems(OnEnter(InGameState::Preparation), despawn_all_enemies)
        .register_type::<Enemy>(); // used for debug inspection
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
    /// Colour of the enemy
    pub colour: Color,
    /// Health of the enemy
    pub health: u8,
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
            colour: Color::srgb(255.0, 0.0, 0.0), // override later
            health: 4,
        }
    }
}
