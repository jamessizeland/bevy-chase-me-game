//! The title screen that appears when the game starts.

use super::{enter_credits, enter_prep};
use crate::prelude::*;

const STATE: Screen = Screen::Title;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(STATE), show_title_screen);
    app.add_systems(OnExit(STATE), stop_bgm);
}

fn show_title_screen(mut commands: Commands) {
    let enter_prep = commands.register_one_shot_system(enter_prep);
    let enter_credits = commands.register_one_shot_system(enter_credits);
    #[cfg(not(target_family = "wasm"))]
    let exit_app = commands.register_one_shot_system(exit_app);

    commands
        .ui_root()
        .insert(StateScoped(STATE))
        .with_children(|children| {
            children.button("Play", enter_prep, Some(KeyCode::KeyP));
            children.button("Credits", enter_credits, Some(KeyCode::KeyC));

            #[cfg(not(target_family = "wasm"))]
            children.button("Exit", exit_app, Some(KeyCode::Escape));
        });
    commands.play_bgm(BgmHandles::PATH_TITLES);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}

fn stop_bgm(mut commands: Commands) {
    commands.stop_bgm();
}
