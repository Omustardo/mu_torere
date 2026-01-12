//! Mū Tōrere - Traditional Māori strategy game.
//!
//! A two-player abstract strategy game played on a board with 8 outer positions
//! (kewai) arranged in a circle, connected to a central position (pūtahi).

mod animation;
mod board;
mod computer;
mod input;
pub mod state;
mod ui;

use bevy::prelude::*;

pub use state::{GameMode, GameSettings};

use crate::screens::{ActiveGame, Screen};

/// Returns true if Mu Torere is the active game being played
pub fn is_playing_mu_torere(screen: Res<State<Screen>>) -> bool {
    *screen.get() == Screen::Playing(ActiveGame::MuTorere)
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        board::plugin,
        computer::plugin,
        input::plugin,
        state::plugin,
        ui::plugin,
    ));
}
