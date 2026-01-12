//! The game selection menu (main menu hub).

use bevy::prelude::*;

use crate::{menus::Menu, screens::ActiveGame, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::GameSelect), spawn_game_select_menu);
    // Track which game is selected for the options menu
    app.init_resource::<SelectedGame>();
}

/// Resource to track which game the player selected (for the options menu)
#[derive(Resource, Default)]
pub struct SelectedGame {
    pub game: Option<ActiveGame>,
}

fn spawn_game_select_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Game Select Menu"),
        GlobalZIndex(2),
        StateScoped::new(Menu::GameSelect),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::header("Select a Game"),
            widget::button("Mū Tōrere", select_mu_torere),
            widget::button("Settings", open_settings_menu),
            widget::button("Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::header("Select a Game"),
            widget::button("Mū Tōrere", select_mu_torere),
            widget::button("Settings", open_settings_menu),
        ],
    ));
}

fn select_mu_torere(
    _: On<Pointer<Click>>,
    mut selected: ResMut<SelectedGame>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    selected.game = Some(ActiveGame::MuTorere);
    next_menu.set(Menu::GameOptions);
}

fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
