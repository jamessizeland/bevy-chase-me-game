use bevy::{input::ButtonInput, prelude::*};

use crate::{components, resources};

pub fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut money: ResMut<resources::Money>,
    player: Query<&Transform, With<components::Player>>,
) {
    let pig_cost = 10.0;
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    let player_transform = player.single();

    if money.0 < pig_cost {
        return;
    }
    money.0 -= pig_cost;
    info!(
        "Spent ${} on a pig, remaining money: ${}",
        pig_cost, money.0
    );
    let texture = asset_server.load("pig.png");
    commands.spawn((
        SpriteBundle {
            texture,
            transform: *player_transform,
            ..default()
        },
        components::Pig {
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
        },
    ));
}

pub fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut components::Pig)>,
    mut money: ResMut<resources::Money>,
) {
    for (entity, mut pig) in pigs.iter_mut() {
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished() {
            money.0 += 15.0;
            commands.entity(entity).despawn();
            info!("Pig sold, gained $15, remaining money: ${}", money.0);
        }
    }
}
