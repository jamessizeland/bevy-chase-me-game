//! The game's main screen states and transitions between them.

mod credits;
mod loading;
mod pause;
mod playing;
mod prep;
mod splash;
mod summary;
mod title;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
        credits::plugin,
        playing::plugin,
        pause::plugin,
        prep::plugin,
        summary::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    /// The splash screen that appears when the game starts.
    #[default]
    Splash,
    /// The loading screen that appears when the game is loading.
    Loading,
    /// The title screen that appears when the game starts.
    Title,
    /// The credits screen that appears when the player selects the credits option.
    Credits,
    /// The prepare screen that appears when the player starts preparing to play the game.
    Preparation,
    /// The playing screen that appears when the player starts playing the game.
    Playing,
    /// The pause screen that appears when the player pauses the game.
    Paused,
    /// The game over screen that appears when the player loses the game.
    Summary,
}

/// Transition to the title screen.
pub fn enter_title(
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    next_screen.set(Screen::Title);
    next_state.set(InGameState::None);
}

/// Transition to the playing screen.
pub fn enter_game(
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    next_screen.set(Screen::Playing);
    next_state.set(InGameState::Playing);
}

/// Transition to the prepare screen.
pub fn enter_prep(
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    next_screen.set(Screen::Preparation);
    next_state.set(InGameState::Preparation);
}

/// Transition to the credits screen.
pub fn enter_credits(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Credits);
}

/// Transition to the pause screen.
pub fn enter_pause(
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    next_screen.set(Screen::Paused);
    next_state.set(InGameState::Paused);
}
