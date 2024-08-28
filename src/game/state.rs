use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum InGameState {
    #[default]
    None,
    Preparation,
    Playing,
    Paused,
    Summary,
}

pub(super) fn plugin(app: &mut App) {
    app
        // Initialize the Game State Machine
        .init_state::<InGameState>()
        // Initialize Game Events
        .add_event::<RestartRequested>()
        .add_event::<TogglePauseRequested>()
        .add_event::<EndGameTriggered>()
        // .add_event::<OptionsRequested>()
        // .add_event::<UpdateOptions>()
        .add_systems(
            Update,
            check_summary_condition
                .in_set(AppSet::UpdateScore) // Update the score
                .run_if(in_state(InGameState::Playing)),
        )
        .add_systems(OnEnter(InGameState::Preparation), clean_up);
}

/// Clean up the game state when the game is over
fn clean_up(mut commands: Commands, mut next_state: ResMut<NextState<InGameState>>) {
    next_state.set(InGameState::None);
    commands.insert_resource(Score::default());
    commands.insert_resource(GameTime::default());
}

pub fn check_summary_condition(
    mut end_game_events: EventReader<EndGameTriggered>,
    mut next_state: ResMut<NextState<InGameState>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if end_game_events.is_empty() {
        return;
    }

    end_game_events.clear();
    next_state.set(InGameState::Summary);
    next_screen.set(Screen::Summary);
}
