use bevy::asset::AssetServer;
use bevy::prelude::*;
use crate::common::better_button::{ColorButton, ReleaseButton, ReleaseButtonForceKey};

pub fn spawn_full_screen_menu_button<T : Default>(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    label: impl Into<String>,
    key_code: KeyCode,
) where (T, ReleaseButton, ReleaseButtonForceKey, ColorButton, ButtonBundle): bevy::prelude::Bundle
{
    parent.spawn(
        (
            T::default(),
            ReleaseButton::default(),
            ReleaseButtonForceKey::new(key_code),
            ColorButton::default(),
            ButtonBundle {
                image: UiImage {
                    texture: asset_server.load("sprites/buttonDefault.png"),
                    ..default()
                },
                style: Style {
                    height: Val::Px(49.),
                    width: Val::Px(190.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            }
        )
    ).with_children(|parent| {
        parent.spawn(
            TextBundle::from_section(format!("{} [{:?}]", label.into(), key_code), TextStyle {
                font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                font_size: 32.,
                color: Color::BLACK,
            }).with_style(Style {
                margin: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(0.), Val::Px(6.)),
                ..default()
            })
        );
    });
}

pub fn spawn_full_screen_menu_header(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    label: impl Into<String>,
)
{
    parent.spawn(
        TextBundle::from_section(label, TextStyle {
            font: asset_server.load("fonts/OpenSans-Regular.ttf"),
            font_size: 90.,
            color: Color::WHITE,
        })
    );
}

pub fn get_full_screen_menu_node_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(12.),
            ..default()
        },
        ..default()
    }
}