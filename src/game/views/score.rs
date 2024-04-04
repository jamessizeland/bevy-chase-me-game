use crate::game::resources::Score;
use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreView;
#[derive(Component)]
pub struct ScoreText;

pub fn spawn_score_view(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            ScoreView {},
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Auto,
                    height: Val::Px(32.),
                    top: Val::Px(12.),
                    left: Val::Px(12.),
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    height: Val::Percent(75.),
                    ..default()
                },
                image: UiImage {
                    texture: asset_server.load("sprites/collectables/element_blue_square.png"),
                    ..default()
                },
                ..default()
            });
            parent.spawn((
                ScoreText {},
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "x 0",
                        TextStyle {
                            font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                            font_size: 30.,
                            color: Color::BLACK,
                            ..default()
                        },
                    ),
                    ..default()
                },
            ));
        });
}

pub fn despawn_score_view(mut commands: Commands, view_query: Query<Entity, With<ScoreView>>) {
    for view in view_query.iter() {
        commands.entity(view).despawn_recursive();
    }
}

pub fn update_score_view(
    score: Res<Score>,
    mut indicator_query: Query<&mut Text, With<ScoreText>>,
) {
    if !score.is_changed() {
        return;
    }

    for mut indicator in indicator_query.iter_mut() {
        indicator.sections[0].value = format!("x {}", score.0);
    }
}
