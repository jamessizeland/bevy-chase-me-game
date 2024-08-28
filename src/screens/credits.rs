//! A credits screen that can be accessed from the title screen.

use super::enter_title;
use crate::prelude::*;

const STATE: Screen = Screen::Credits;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(STATE), show_credits_screen);
    app.add_systems(OnExit(STATE), stop_bgm);
}

fn show_credits_screen(mut commands: Commands) {
    let enter_title = commands.register_one_shot_system(enter_title);

    commands
        .ui_root()
        .insert(StateScoped(STATE))
        .with_children(|children| {
            children.header("Made by");
            children.label("James Sizeland");
            children.label("based on the BevyFlock template");

            children.button("Back", enter_title, Some(KeyCode::Escape));
        });

    commands.play_bgm(BgmHandles::PATH_CREDITS);
}

fn stop_bgm(mut commands: Commands) {
    commands.stop_bgm();
}
