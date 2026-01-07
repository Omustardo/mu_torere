//! The new game menu for selecting game mode.

use bevy::prelude::*;

use crate::{
    asset_tracking::ResourceHandles,
    game::{GameMode, GameSettings},
    menus::Menu,
    screens::Screen,
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::NewGame), spawn_new_game_menu);
}

fn spawn_new_game_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("New Game Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::NewGame),
        children![
            widget::header("New Game"),
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
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
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
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn go_back(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}
