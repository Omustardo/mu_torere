//! Game options menu - shown after selecting a game, before starting it.
//! Displays game-specific options (e.g., vs Player / vs Computer for Mu Torere).

use bevy::prelude::*;

use crate::{
    asset_tracking::ResourceHandles,
    games::mu_torere::{GameMode, GameSettings},
    menus::Menu,
    screens::{ActiveGame, Screen},
    theme::widget,
};

use super::game_select::SelectedGame;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::GameOptions), spawn_game_options_menu);
}

fn spawn_game_options_menu(mut commands: Commands, selected: Res<SelectedGame>) {
    let Some(game) = selected.game else {
        return;
    };

    match game {
        ActiveGame::MuTorere => spawn_mu_torere_options(&mut commands),
        ActiveGame::MentalMath => spawn_mental_math_options(&mut commands),
    }
}

fn spawn_mu_torere_options(commands: &mut Commands) {
    commands.spawn((
        widget::ui_root("Mu Torere Options"),
        GlobalZIndex(2),
        StateScoped(Menu::GameOptions),
        children![
            widget::header("Mū Tōrere"),
            widget::button("vs Player", start_vs_player),
            widget::button("vs Computer", start_vs_computer),
            widget::button("Back", go_back),
        ],
    ));
}

fn start_vs_player(
    _: On<Pointer<Click>>,
    mut settings: ResMut<GameSettings>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    settings.mode = GameMode::VsPlayer;
    next_menu.set(Menu::None);
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Playing(ActiveGame::MuTorere));
    } else {
        next_screen.set(Screen::Loading(ActiveGame::MuTorere));
    }
}

fn start_vs_computer(
    _: On<Pointer<Click>>,
    mut settings: ResMut<GameSettings>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    settings.mode = GameMode::VsComputer;
    next_menu.set(Menu::None);
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Playing(ActiveGame::MuTorere));
    } else {
        next_screen.set(Screen::Loading(ActiveGame::MuTorere));
    }
}

fn go_back(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::GameSelect);
}

fn spawn_mental_math_options(commands: &mut Commands) {
    commands.spawn((
        widget::ui_root("Mental Math Options"),
        GlobalZIndex(2),
        StateScoped(Menu::GameOptions),
        children![
            widget::header("Mental Math"),
            widget::label("Solve math problems!"),
            widget::button("Start", start_mental_math),
            widget::button("Back", go_back),
        ],
    ));
}

fn start_mental_math(
    _: On<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    next_menu.set(Menu::None);
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Playing(ActiveGame::MentalMath));
    } else {
        next_screen.set(Screen::Loading(ActiveGame::MentalMath));
    }
}
