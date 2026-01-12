//! The screen state for the main gameplay.
//! This module handles generic gameplay functionality that applies to all games.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    menus::Menu,
    screens::{is_playing, ActiveGame, Screen},
    Pause,
};

pub(super) fn plugin(app: &mut App) {
    // Pause/unpause handling - runs for any game
    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                is_playing
                    .and(in_state(Menu::None))
                    .and(input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape))),
            ),
            close_menu.run_if(
                is_playing
                    .and(not(in_state(Menu::None)))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
        ),
    );

    // Clean up when leaving any game
    app.add_systems(
        OnExit(Screen::Playing(ActiveGame::MuTorere)),
        (close_menu, unpause),
    );
    app.add_systems(
        OnExit(Screen::Playing(ActiveGame::MentalMath)),
        (close_menu, unpause),
    );

    app.add_systems(OnEnter(Menu::None), unpause.run_if(is_playing));
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(false));
}

fn pause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(true));
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        StateScoped(Pause(true)),
    ));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
