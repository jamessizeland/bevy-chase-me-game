use bevy::prelude::*;

#[derive(Debug, Component)]
struct Position {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_systems(Startup, setup)
        // .add_systems(Update, (update_position, print_position))
        .add_plugins(DefaultPlugins)
        .run();
}

/// This system runs once at the start of the app
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("character.png");

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        ..default()
    });
}

fn print_position(query: Query<(Entity, &Position)>) {
    // log the entity and position of all entities with a Position component
    for (entity, position) in query.iter() {
        info!("{:?} position is {:?}", entity, position);
    }
}
