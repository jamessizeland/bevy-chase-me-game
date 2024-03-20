use bevy::{input::ButtonInput, prelude::*};
use bevy_inspector_egui::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, character_movement)
            .register_type::<Player>(); // used for debug inspection
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    #[inspector(min = 0.0)] // set min value that can be input with bevy-inspector-egui
    pub speed: f32,
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
        transform.translation += translation;
    }
}
