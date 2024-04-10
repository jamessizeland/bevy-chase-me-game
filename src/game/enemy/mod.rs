mod resources;
mod systems;

use super::{
    events::{ShipDestroyed, ShipHit},
    state::InGameState,
    GameTime, Score,
};
use crate::AppState;
use bevy::prelude::*;
use rand::Rng;
use resources::EnemyStrengthRange;
use systems::{enemy_lifetime, spawn_enemy};

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
        app.init_resource::<EnemyStrengthRange>()
            .add_event::<ShipDestroyed>()
            .add_event::<ShipHit>()
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
            colour: Color::RED, // override later
            health: 4,
        }
    }
    /// Set the colour of the enemy
    pub fn set_colour(&mut self, colour: Color) {
        self.colour = colour;
    }
}
