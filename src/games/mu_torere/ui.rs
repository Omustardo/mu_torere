//! UI components specific to Mu Torere (turn indicator, game over screen).

use bevy::prelude::*;

use crate::{
    menus::Menu,
    screens::{ActiveGame, Screen},
    theme::widget,
};

use super::{
    is_playing_mu_torere,
    state::{GameOverEvent, GameState, PieceColor},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Playing(ActiveGame::MuTorere)),
        reset_game_state,
    );
    app.add_systems(
        Update,
        (spawn_turn_indicator, update_turn_indicator).run_if(is_playing_mu_torere),
    );
    app.add_systems(Update, handle_game_over.run_if(is_playing_mu_torere));
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
            top: Val::Px(20.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        StateScoped(Screen::Playing(ActiveGame::MuTorere)),
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
        for child in children.iter() {
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
    mut game_over_events: MessageReader<GameOverEvent>,
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
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            GlobalZIndex(3),
            StateScoped(Screen::Playing(ActiveGame::MuTorere)),
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
    next_screen.set(Screen::MainMenu);
    next_menu.set(Menu::GameSelect);
}
