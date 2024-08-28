//! A credits screen that can be accessed from the title screen.

use super::{enter_game, enter_title};
use crate::game::level::SpawnLevel;
use crate::prelude::*;

const STATE: Screen = Screen::Preparation;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(STATE), show_prep_screen)
        .add_systems(OnExit(STATE), spawn_level);
}

fn show_prep_screen(mut commands: Commands) {
    let enter_game = commands.register_one_shot_system(enter_game);
    let enter_title = commands.register_one_shot_system(enter_title);

    commands
        .ui_root()
        .insert(StateScoped(STATE))
        .with_children(|children| {
            children.header("Ready to play?");

            children.label("Use arrows or WASD to move the player.");
            children.label("Escape the enemies");

            children.button("Start", enter_game, Some(KeyCode::Space));
            children.button("Back", enter_title, Some(KeyCode::Escape));
        });
}

fn spawn_level(mut commands: Commands) {
    commands.add(SpawnLevel);
    commands.play_bgm(
        get_random_array_element(&[BgmHandles::PATH_GAMEPLAY1, BgmHandles::PATH_GAMEPLAY2])
            .to_string(),
    );
}

fn get_random_array_element<T>(array: &[T]) -> &T {
    let mut rng = rand::thread_rng();
    let range = rand::distributions::Uniform::new(0, array.len());
    let index = rng.sample(range);
    &array[index]
}
