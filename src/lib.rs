mod assets;
mod audio;
#[cfg(feature = "dev")]
mod dev_tools;
mod game;
mod screens;
mod theme;

use crate::prelude::*;
use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
};
use bevy_trauma_shake::TraumaPlugin;

mod prelude {
    pub use crate::{
        assets::{BgmHandles, SfxHandles},
        audio::bgm::BgmCommands as _,
        game::{events::*, resources::*, state::InGameState},
        screens::Screen,
        theme::prelude::*,
    };
    pub use bevy::prelude::*;
    pub use bevy_prototype_lyon::prelude::*;
    pub use bevy_rapier2d::prelude::*;
    pub use bevy_trauma_shake::prelude::*;
    pub use rand::{distributions::Uniform, Rng as _};

    /// High-level groupings of systems for the app in the `Update` schedule.
    /// NOTE: When adding a new variant, make sure to order it in the `configure_sets`
    /// call below.
    #[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
    pub enum AppSet {
        /// Tick timers.
        TickTimers,
        /// Record player input.
        RecordInput,
        /// Do everything else (consider splitting this into further variants).
        Update,
        /// Check score and end game conditions.
        UpdateScore,
    }
    /// Convert velocity in x and y to a single scaled magnitude value.
    pub fn get_magnitude(velocity: &Velocity) -> f32 {
        (velocity.linvel[0].abs() + velocity.linvel[1].abs()) * 0.08
    }
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSet::TickTimers,
                AppSet::RecordInput,
                AppSet::Update,
                AppSet::UpdateScore,
            )
                .chain(),
        );

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Chase Me".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        focused: true,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                }),
        );

        // Add other plugins.
        app.add_plugins((
            game::plugin,
            screens::plugin,
            theme::plugin,
            assets::plugin,
            audio::plugin,
            TraumaPlugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle::default(),
        Shake::default(),
        ShakeSettings {
            decay_per_second: 2.0, // up from 0.8
            amplitude: 8.0,        // down from 100.0
            ..default()
        },
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
    ));
}
