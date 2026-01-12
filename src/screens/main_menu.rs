//! The main menu screen for game selection.

use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::MainMenu), open_game_select_menu);
    app.add_systems(OnExit(Screen::MainMenu), close_menu);
}

fn open_game_select_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::GameSelect);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
