use super::PlayerSpeed;
use crate::window::{WINDOW_USABLE_WORLD_WIDTH, WINDOW_WORLD_HEIGHT};
use bevy::{input::ButtonInput, prelude::*};
use bevy_inspector_egui::prelude::*;

// pub struct PlayerPlugin;

// impl Plugin for PlayerPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Update, (character_movement,))
//             .register_type::<Player>(); // used for debug inspection
//     }
// }

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    #[inspector(min = 0.0)] // set min value that can be input with bevy-inspector-egui
    pub speed: f32,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_speed: Res<PlayerSpeed>,
) {
    let character = asset_server.load("sprites/character.png");
    commands.spawn((
        SpriteBundle {
            texture: character,
            transform: Transform {
                translation: Vec3::new(
                    WINDOW_USABLE_WORLD_WIDTH / 2.0,
                    WINDOW_WORLD_HEIGHT / 2.0,
                    0.0,
                ),
                scale: Vec3::new(2.0, 2.0, 1.0),
                ..default()
            },
            ..default()
        },
        Player {
            speed: player_speed.get_speed(),
        },
        Name::new("Player"),
    ));
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Move the player character based on input. Normalized diagonal movement to keep speed consistent in all directions.
pub fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in characters.iter_mut() {
        let movement_amount = player.speed * time.delta_seconds();
        let mut translation = Vec3::ZERO;
        if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
            translation.y += movement_amount;
        }
        if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
            translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
            translation.x -= movement_amount;
        }
        if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
            translation.x += movement_amount;
        }
        if translation.x != 0.0 && translation.y != 0.0 {
            translation /= f32::sqrt(2.0); // Normalize diagonal movement
        }
        let player_width = transform.scale.x;
        let player_height = transform.scale.y;
        let player_half_width = player_width / 2.0;
        let player_half_height = player_height / 2.0;
        let mut new_translation = transform.translation + translation;
        new_translation.x = new_translation.x.clamp(
            player_half_width,
            WINDOW_USABLE_WORLD_WIDTH - player_half_width,
        );
        new_translation.y = new_translation
            .y
            .clamp(player_half_height, WINDOW_WORLD_HEIGHT - player_half_height);
        transform.translation = new_translation;
    }
}
