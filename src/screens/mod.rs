//! The game's main screen states and transitions between them.

mod gameplay;
mod loading;
mod main_menu;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((gameplay::plugin, loading::plugin, main_menu::plugin));
}

/// The game being played (used as data in Screen variants).
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum ActiveGame {
    MuTorere,
    MentalMath,
}

/// The game's main screen states.
#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    /// Main menu / game selection screen
    #[default]
    MainMenu,
    /// Loading assets for a specific game
    Loading(ActiveGame),
    /// Playing a specific game
    Playing(ActiveGame),
}

/// Helper function to check if we're playing any game
pub fn is_playing(screen: Res<State<Screen>>) -> bool {
    matches!(screen.get(), Screen::Playing(_))
}

/// Helper function to check if we're loading any game
pub fn is_loading(screen: Res<State<Screen>>) -> bool {
    matches!(screen.get(), Screen::Loading(_))
}
