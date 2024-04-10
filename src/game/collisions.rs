//! Plugin to handle collisions between entities

use bevy::{prelude::*, utils::hashbrown::Equivalent};
use bevy_rapier2d::prelude::*;

use crate::game::events::EndGameTriggered;

use super::{enemy::Enemy, events::ShipHit, player::Player, state::InGameState};

/// Plugin to handle collisions between entities
pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            collision_events.run_if(in_state(InGameState::Play)),
        );
    }
}

/// Trigger an end of game event when the player gets hit by an enemy
fn collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_destroyed: EventWriter<EndGameTriggered>,
    mut ship_hit_events: EventWriter<ShipHit>,
    players: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
) {
    let player = players.single();
    for collision_event in collision_events.read() {
        for enemy in enemies.iter() {
            if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
                if (entity1.equivalent(&player) && entity2.equivalent(&enemy))
                    || (entity2.equivalent(&player) && entity1.equivalent(&enemy))
                {
                    info!("Player collided with enemy");
                    player_destroyed.send(EndGameTriggered);
                }
            }
        }
    }
}
