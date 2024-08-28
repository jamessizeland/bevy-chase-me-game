//! The screen state for the main game loop.

use super::enter_pause;
use crate::prelude::*;
use bevy::input::common_conditions::input_just_pressed;
use ui_palette::LABEL_TEXT;

const STATE: Screen = Screen::Playing;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnExit(STATE), stop_bgm);
    app.add_systems(OnEnter(STATE), add_score_ui);

    app.add_systems(
        Update,
        enter_pause.run_if(in_state(STATE).and_then(input_just_pressed(KeyCode::Escape))),
    );
    app.add_systems(
        Update,
        update_score
            .in_set(AppSet::UpdateScore)
            .run_if(in_state(InGameState::Playing)),
    );
}

fn stop_bgm(mut commands: Commands) {
    commands.stop_bgm();
}

#[derive(Component, Debug)]
struct ScoreIndicator;

// System to update the text with the current score
fn update_score(
    mut query: Query<&mut Text, With<ScoreIndicator>>,
    score: Res<Score>,
    runtime: Res<GameTime>,
) {
    for mut text in query.iter_mut() {
        // Update the text with the current score
        text.sections[0].value =
            format!("Score: {} | Runtime: {:.1}s", score.0 as u32, runtime.time);
    }
}

fn add_score_ui(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(STATE))
        .with_children(|children| {
            let mut entity = children.spawn((
                Name::new("Score"),
                NodeBundle {
                    style: Style {
                        width: Val::Px(500.0),
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::End,
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    ..default()
                },
            ));
            entity.with_children(|children| {
                children.spawn((
                    Name::new("Score Text"),
                    ScoreIndicator,
                    TextBundle::from_section(
                        "Score: 0 | Runtime: 0.0s",
                        TextStyle {
                            font_size: 24.0,
                            color: LABEL_TEXT,
                            ..default()
                        },
                    ),
                ));
            });
        });
}
