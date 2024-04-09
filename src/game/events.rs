use bevy::prelude::*;

/// Event for requesting a restart of the game.
#[derive(Event, Default)]
pub struct RestartRequested;

/// Event for requesting the main menu.
#[derive(Event, Default)]
pub struct MenuRequested;

/// Event for requesting a pause toggle.
#[derive(Event, Default)]
pub struct TogglePauseRequested;

/// Event for the end of the game.
#[derive(Event, Default)]
pub struct EndGameTriggered;

/// Event for requesting the options menu.
#[derive(Event, Default)]
pub struct OptionsRequested;

/// Event for updating game options.
#[derive(Event, Default)]
pub struct UpdateOptions;
