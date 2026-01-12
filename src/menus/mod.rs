//! The game's menus and transitions between them.

mod game_options;
mod game_select;
mod pause;
mod settings;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Menu>();

    app.add_plugins((
        game_select::plugin,
        game_options::plugin,
        settings::plugin,
        pause::plugin,
    ));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Menu {
    #[default]
    None,
    /// Game selection menu (main menu)
    GameSelect,
    /// Game-specific options (e.g., vs Player / vs Computer for Mu Torere)
    GameOptions,
    /// Global settings
    Settings,
    /// In-game pause menu
    Pause,
}
