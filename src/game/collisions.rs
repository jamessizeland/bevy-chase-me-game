//! Plugin to handle collisions between entities

use super::{
    enemy::{Enemy, EnemyState},
    events::ShipHit,
    player::Player,
};
use crate::{audio::sfx::SfxCommands, prelude::*};
use bevy::{
    audio::{PlaybackMode, Volume},
    utils::hashbrown::Equivalent,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        collision_events
            .in_set(AppSet::Update)
            .run_if(in_state(InGameState::Playing)),
    );
}

/// Trigger an end of game event when the player gets hit by an enemy
fn collision_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_destroyed: EventWriter<EndGameTriggered>,
    mut ship_hit_events: EventWriter<ShipHit>,
    players: Query<(Entity, &Velocity), With<Player>>,
    enemies: Query<(Entity, &Enemy)>,
) {
    let (player, player_vel) = players
        .get_single()
        .expect("Collision system found more than one player, this should not happen");

    for collision_event in collision_events.read() {
        commands.play_sfx_with_settings(
            SfxHandles::PATH_SELECT,
            PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::new(0.5),
                speed: randomise_speed(),
                ..default()
            },
        );
        for (enemy_entity, enemy) in enemies.iter() {
            if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
                if entity1.equivalent(&enemy_entity) {
                    ship_hit_events.send(ShipHit { id: *entity1 });
                    if entity2.equivalent(&player) && enemy.state == EnemyState::Moving {
                        info!("Player collided with enemy");
                        player_destroyed.send(EndGameTriggered);
                    }
                } else if entity2.equivalent(&enemy_entity) && enemy.state == EnemyState::Moving {
                    ship_hit_events.send(ShipHit { id: *entity2 });
                    if entity1.equivalent(&player) {
                        info!("Player collided with enemy");
                        player_destroyed.send(EndGameTriggered);
                    }
                } else if entity1.equivalent(&player) || entity2.equivalent(&player) {
                    let velocity_magnitude = get_magnitude(player_vel);
                    info!(
                        "Player collided with something else with velocity {}",
                        velocity_magnitude
                    );
                    commands.add_trauma(velocity_magnitude);
                }
            }
        }
    }
}

fn randomise_speed() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.5..1.5)
}
