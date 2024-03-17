use bevy::{input::ButtonInput, prelude::*};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

/// Move the player character based on input
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
