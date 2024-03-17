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
