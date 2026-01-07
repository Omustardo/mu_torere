//! The screen state for the main gameplay.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    Pause,
    game::state::{GameOverEvent, GameState, PieceColor},
    menus::Menu,
    screens::Screen,
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), reset_game_state);
    app.add_systems(
        Update,
        (spawn_turn_indicator, update_turn_indicator).run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(Update, handle_game_over.run_if(in_state(Screen::Gameplay)));

    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                in_state(Screen::Gameplay)
                    .and(in_state(Menu::None))
                    .and(input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape))),
            ),
            close_menu.run_if(
                in_state(Screen::Gameplay)
                    .and(not(in_state(Menu::None)))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
        ),
    );
    app.add_systems(OnExit(Screen::Gameplay), (close_menu, unpause));
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Screen::Gameplay)),
    );
}

fn reset_game_state(mut game_state: ResMut<GameState>) {
    game_state.reset();
}

#[derive(Component)]
struct TurnIndicator;

#[derive(Component)]
struct GameOverUI;

fn spawn_turn_indicator(
    mut commands: Commands,
    existing: Query<Entity, With<TurnIndicator>>,
    game_state: Res<GameState>,
) {
    if !existing.is_empty() {
        return;
    }

    let text = if game_state.game_over {
        match game_state.winner {
            Some(PieceColor::White) => "White Wins!".to_string(),
            Some(PieceColor::Black) => "Black Wins!".to_string(),
            None => "Game Over".to_string(),
        }
    } else {
        match game_state.current_turn {
            PieceColor::White => "White's Turn".to_string(),
            PieceColor::Black => "Black's Turn".to_string(),
        }
    };

    commands.spawn((
        Name::new("Turn Indicator"),
        TurnIndicator,
        Node {
            position_type: PositionType::Absolute,
            top: px(20),
            left: px(0),
            right: px(0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        DespawnOnExit(Screen::Gameplay),
        children![(
            Text(text),
            TextFont::from_font_size(32.0),
            TextColor(Color::WHITE),
        )],
    ));
}

fn update_turn_indicator(
    game_state: Res<GameState>,
    indicators: Query<&Children, With<TurnIndicator>>,
    mut texts: Query<&mut Text>,
) {
    for children in &indicators {
        for &child in children.iter() {
            if let Ok(mut text) = texts.get_mut(child) {
                text.0 = if game_state.game_over {
                    match game_state.winner {
                        Some(PieceColor::White) => "White Wins!".to_string(),
                        Some(PieceColor::Black) => "Black Wins!".to_string(),
                        None => "Game Over".to_string(),
                    }
                } else {
                    match game_state.current_turn {
                        PieceColor::White => "White's Turn".to_string(),
                        PieceColor::Black => "Black's Turn".to_string(),
                    }
                };
            }
        }
    }
}

fn handle_game_over(
    mut commands: Commands,
    mut game_over_events: EventReader<GameOverEvent>,
    existing: Query<Entity, With<GameOverUI>>,
) {
    for event in game_over_events.read() {
        if !existing.is_empty() {
            continue;
        }

        let winner_text = match event.winner {
            PieceColor::White => "White Wins!",
            PieceColor::Black => "Black Wins!",
        };

        commands.spawn((
            Name::new("Game Over UI"),
            GameOverUI,
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: px(20),
                ..default()
            },
            GlobalZIndex(3),
            DespawnOnExit(Screen::Gameplay),
            children![
                widget::header(winner_text),
                widget::button("Main Menu", return_to_main_menu),
            ],
        ));
    }
}

fn return_to_main_menu(
    _: On<Pointer<Click>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    next_screen.set(Screen::Title);
    next_menu.set(Menu::Main);
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
            width: percent(100),
            height: percent(100),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        DespawnOnExit(Pause(true)),
    ));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
