//! Game state management for Mu Torere.

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GameSettings>();
    app.init_resource::<GameState>();
    app.add_message::<TurnChangeEvent>();
    app.add_message::<GameOverEvent>();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameMode {
    #[default]
    VsPlayer,
    VsComputer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PieceColor {
    #[default]
    White,
    Black,
}

impl PieceColor {
    pub fn opposite(self) -> Self {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

#[derive(Resource, Default)]
pub struct GameSettings {
    pub mode: GameMode,
    pub instant_animation: bool,
}

#[derive(Resource, Default)]
pub struct GameState {
    pub current_turn: PieceColor,
    pub game_over: bool,
    pub winner: Option<PieceColor>,
}

impl GameState {
    pub fn reset(&mut self) {
        self.current_turn = PieceColor::White;
        self.game_over = false;
        self.winner = None;
    }
}

#[derive(Message)]
pub struct TurnChangeEvent {
    pub new_turn: PieceColor,
}

#[derive(Message)]
pub struct GameOverEvent {
    pub winner: PieceColor,
}
