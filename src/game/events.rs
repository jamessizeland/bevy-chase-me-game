use crate::prelude::*;

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

/// Event for when a ship is destroyed, with the x and y coordinates of the ship.
#[derive(Event, Default)]
pub struct ShipDestroyed {
    pub x: f32,
    pub y: f32,
    pub colour: Color,
}

/// Event for when a ship is hit. Contains the amount of damage dealt.
/// and the x and y coordinates of the ship.
#[derive(Event)]
pub struct ShipHit {
    pub id: Entity,
    // pub damage: f32,
    // pub x: f32,
    // pub y: f32,
    // pub colour: Color,
}
