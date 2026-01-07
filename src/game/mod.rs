//! Mu Torere game implementation.

mod animation;
mod board;
mod computer;
mod input;
pub mod state;

use bevy::prelude::*;

pub use state::{GameMode, GameSettings};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        board::plugin,
        computer::plugin,
        input::plugin,
        state::plugin,
    ));
}
