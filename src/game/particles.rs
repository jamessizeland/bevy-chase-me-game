use bevy::prelude::*;
use bevy_particle_systems::{
    ColorOverTime, Curve, CurvePoint, JitteredValue, ParticleBurst, ParticleSystem,
    ParticleSystemBundle, ParticleSystemPlugin, Playing, VelocityModifier::*,
};

use super::events::ShipDestroyed;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ParticleSystemPlugin)
            .add_systems(Update, destroyed_ship);
    }
}

fn destroyed_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut destroyed_ship_events: EventReader<ShipDestroyed>,
) {
    for event in destroyed_ship_events.read() {
        commands
            .spawn(ParticleSystemBundle {
                transform: Transform::from_xyz(event.x, event.y, 0.0),
                particle_system: ParticleSystem {
                    max_particles: 500,
                    texture: asset_server.load("particles/px.png").into(),
                    spawn_rate_per_second: 1000.0.into(),
                    initial_speed: JitteredValue::jittered(200.0, -50.0..50.0),
                    velocity_modifiers: vec![Drag(0.01.into())],
                    lifetime: JitteredValue::jittered(4.0, -2.0..2.0),
                    color: ColorOverTime::Gradient(Curve::new(vec![
                        CurvePoint::new(event.colour, 0.0),
                        CurvePoint::new(Color::WHITE, 0.5),
                        CurvePoint::new(Color::rgba(0.0, 0.0, 1.0, 0.0), 1.0),
                    ])),
                    looping: false,
                    system_duration_seconds: 0.1,
                    max_distance: Some(50.0),
                    scale: 2.0.into(),
                    bursts: vec![
                        ParticleBurst::new(0.0, 100),
                        // ParticleBurst::new(2.0, 100),
                        // ParticleBurst::new(4.0, 100),
                        // ParticleBurst::new(6.0, 100),
                        // ParticleBurst::new(8.0, 100),
                    ],
                    ..ParticleSystem::default()
                },
                ..ParticleSystemBundle::default()
            })
            .insert(Playing);
    }
}
