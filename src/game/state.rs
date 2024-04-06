use super::{events::*, views, GameTime, Score};
use crate::AppState;
use bevy::prelude::*;

pub struct GameStatePlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum InGameState {
    #[default]
    None,
    Preparation,
    Play,
    Pause,
    Summary,
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize the Game State Machine
            .init_state::<InGameState>()
            // Initialize Game Events
            .add_event::<RestartRequested>()
            .add_event::<MenuRequested>()
            .add_event::<TogglePauseRequested>()
            .add_event::<EndGameTriggered>()
            // Restart Game View States
            .add_systems(OnEnter(AppState::RestartInGame), continue_restart_game)
            // Main Game View States
            .add_systems(OnEnter(AppState::InGame), start_up)
            .add_systems(OnExit(AppState::InGame), clean_up)
            // Prep View States
            .add_systems(
                OnEnter(InGameState::Preparation),
                views::spawn_preparation_view,
            )
            .add_systems(
                OnExit(InGameState::Preparation),
                views::despawn_preparation_view,
            )
            // Pause View States
            .add_systems(OnEnter(InGameState::Pause), views::spawn_pause_view)
            .add_systems(OnExit(InGameState::Pause), views::despawn_pause_view)
            // Summary View States
            .add_systems(OnEnter(InGameState::Summary), views::spawn_summary_view)
            .add_systems(OnExit(InGameState::Summary), views::despawn_summary_view)
            // Main Game Loop
            .add_systems(
                Update,
                (check_preparation_end_condition).run_if(in_state(InGameState::Preparation)),
            )
            .add_systems(
                Update,
                (views::update_score_view).run_if(in_state(InGameState::Play)),
            )
            .add_systems(
                Update,
                (views::check_pause_interactions).run_if(in_state(InGameState::Pause)),
            )
            .add_systems(
                Update,
                (views::check_summary_interactions,).run_if(in_state(InGameState::Summary)),
            )
            .add_systems(
                Update,
                ((
                    check_menu_condition,
                    check_restart_condition,
                    check_toggle_pause_condition,
                    check_summary_condition,
                ),)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

/// Initialize the Game
fn start_up(mut next_state: ResMut<NextState<InGameState>>) {
    next_state.set(InGameState::Preparation);
}

/// Clean up the game state when the game is over
fn clean_up(mut commands: Commands, mut next_state: ResMut<NextState<InGameState>>) {
    next_state.set(InGameState::None);
    commands.insert_resource(Score::default());
    commands.insert_resource(GameTime::default());
}

pub fn check_preparation_end_condition(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    if let Some(key) = keyboard_input.get_just_pressed().next() {
        if *key != KeyCode::ArrowLeft && *key != KeyCode::ArrowRight {
            next_state.set(InGameState::Play);
        }
    } else if mouse_input.get_just_pressed().next() != None {
        next_state.set(InGameState::Play);
    }
}

pub fn check_menu_condition(
    mut menu_requested_events: EventReader<MenuRequested>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if menu_requested_events.is_empty() {
        return;
    }

    menu_requested_events.clear();
    next_state.set(AppState::Menu);
}

pub fn check_restart_condition(
    mut restart_requested_events: EventReader<RestartRequested>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if restart_requested_events.is_empty() {
        return;
    }

    restart_requested_events.clear();
    next_state.set(AppState::RestartInGame);
}

pub fn continue_restart_game(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame);
}

pub fn check_summary_condition(
    mut end_game_events: EventReader<EndGameTriggered>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    if end_game_events.is_empty() {
        return;
    }

    end_game_events.clear();
    next_state.set(InGameState::Summary);
}

pub fn check_toggle_pause_condition(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<InGameState>>,
    mut next_state: ResMut<NextState<InGameState>>,
    mut toggle_pause_requested_events: EventReader<TogglePauseRequested>,
) {
    let mut toggle = false;

    if !toggle_pause_requested_events.is_empty() {
        toggle = true;
        toggle_pause_requested_events.clear();
    }
    if input.just_pressed(KeyCode::Escape) {
        toggle = true;
    }

    if !toggle {
        return;
    }

    if *current_state.get() == InGameState::Play {
        next_state.set(InGameState::Pause);
    } else {
        next_state.set(InGameState::Play);
    }
}
