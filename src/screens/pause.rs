//! The pause screen that appears when the game is paused.

use super::{enter_game, enter_prep, enter_title};
use crate::prelude::*;

const STATE: Screen = Screen::Paused;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(STATE), show_pause_screen);
}

fn show_pause_screen(mut commands: Commands) {
    let enter_playing = commands.register_one_shot_system(enter_game);
    let enter_prep = commands.register_one_shot_system(enter_prep);
    let enter_title = commands.register_one_shot_system(enter_title);

    commands
        .ui_root()
        .insert(StateScoped(STATE))
        .with_children(|children| {
            children.button("Continue", enter_playing, Some(KeyCode::Escape));
            children.button("Restart", enter_prep, Some(KeyCode::KeyR));
            children.button("Menu", enter_title, Some(KeyCode::KeyM));
        });
}
