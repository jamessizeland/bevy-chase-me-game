//! A summary screen that shows when the game ends.

use super::{enter_prep, enter_title};
use crate::prelude::*;

const STATE: Screen = Screen::Summary;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(STATE), show_summary_screen);
    app.add_systems(OnExit(STATE), stop_bgm);
}

fn show_summary_screen(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    let restart_game = commands.register_one_shot_system(enter_prep);
    let enter_title = commands.register_one_shot_system(enter_title);
    let score = score.0.floor() as u32;
    commands
        .ui_root()
        .insert(StateScoped(STATE))
        .with_children(|children| {
            children.header("Game Over");
            children.label(format!("Your score: {}", score));

            // display a sarcastic message if the player gets no points
            if score == 0 {
                children.label(zero_points_message());
            }

            children.button("Again", restart_game, Some(KeyCode::Space));
            children.button("Menu", enter_title, Some(KeyCode::Escape));
        });
    commands.play_bgm(BgmHandles::PATH_CREDITS);
}

fn zero_points_message() -> &'static str {
    // list of sarcastic messages to display when the player gets no points
    let zero_points_message = vec![
        "Wow, you're really good at this game!",
        "You're a natural!",
        "Are you trying?",
        "Seems unlikely...",
        "Again?",
        "Try not getting hit next time!",
        "Try not to die!",
        "Again?",
        "Once more, with feeling!",
        "Fly casual!",
        "When life gives you lemons...",
        "Time to git gud!",
        "What happened?",
        "Oh no happened!",
        "Give it another go!",
        "not like this...",
        "Ouch...",
        "I think you can do better!",
        "You're better than this!",
        "You can do it!",
        "You're a star!",
        "You're a shooting star!",
        "You've got the power!",
        "You're indestructible!",
        "Always believe in yourself!",
        "Hmm...",
        "Go on, try again!",
        "Go on, go on, go on!",
        "Give up?",
        "Thirsty for more?",
        "Just a little more!",
    ];
    let dist = Uniform::new(0, zero_points_message.len());
    let mut rng = rand::thread_rng();
    zero_points_message[rng.sample(&dist)]
}

fn stop_bgm(mut commands: Commands) {
    commands.stop_bgm();
}
