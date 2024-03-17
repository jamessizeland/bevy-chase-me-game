//! # Plugins
//!
//! As your project grows, it can be useful to make it more modular. You can split it into "plugins".
//!
//! Plugins are simply collections of things to be added to the App Builder. Think of this as a way to add things to the app from multiple places, like different Rust files/modules or crates.
//!
//! The simplest way to create a plugin is by just writing a Rust function that takes &mut App:
//!
//! ```
//! fn my_plugin(app: &mut App) {
//!     app.init_resource::<MyCustomResource>();
//!     app.add_systems(Update, (
//!         do_some_things,
//!         do_other_things,
//!     ));
//! }
//! ```

pub mod pig;
pub mod player;

use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    render::texture::ImagePlugin,
    window::{Window, WindowPlugin},
    DefaultPlugins,
};

pub fn app_plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Logic Farming Roguelike".into(),
                resolution: (640.0, 480.0).into(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        })
        .build()
}
