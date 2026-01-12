//! Mental Math game - solve math problems with multiple choice answers.

mod problem;
mod ui;

use bevy::prelude::*;

use crate::screens::{ActiveGame, Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((problem::plugin, ui::plugin));
}

/// Helper function to check if we're playing Mental Math
pub fn is_playing_mental_math(screen: Res<State<Screen>>) -> bool {
    matches!(screen.get(), Screen::Playing(ActiveGame::MentalMath))
}
