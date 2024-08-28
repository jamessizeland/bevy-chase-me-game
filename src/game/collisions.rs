//! Plugin to handle collisions between entities

use super::{enemy::Enemy, events::ShipHit, player::Player};
use crate::prelude::*;
use bevy::utils::hashbrown::Equivalent;

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
    mut collision_events: EventReader<CollisionEvent>,
    mut player_destroyed: EventWriter<EndGameTriggered>,
    mut ship_hit_events: EventWriter<ShipHit>,
    players: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
) {
    let player = players
        .get_single()
        .expect("Collision system found more than one player, this should not happen");

    for collision_event in collision_events.read() {
        for enemy in enemies.iter() {
            if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
                if entity1.equivalent(&enemy) {
                    ship_hit_events.send(ShipHit { id: *entity1 });
                    if entity2.equivalent(&player) {
                        info!("Player collided with enemy");
                        player_destroyed.send(EndGameTriggered);
                    }
                } else if entity2.equivalent(&enemy) {
                    ship_hit_events.send(ShipHit { id: *entity2 });
                    if entity1.equivalent(&player) {
                        info!("Player collided with enemy");
                        player_destroyed.send(EndGameTriggered);
                    }
                };
            }
        }
    }
}
