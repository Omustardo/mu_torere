//! Computer player AI for Mu Torere.

use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::screens::Screen;

use super::{
    animation::MoveEvent,
    board::{Piece, get_valid_moves},
    state::{GameMode, GameSettings, GameState, PieceColor},
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ComputerThinkTimer>();
    app.add_systems(Update, computer_turn.run_if(in_state(Screen::Gameplay)));
}

#[derive(Resource)]
pub struct ComputerThinkTimer {
    timer: Timer,
    thinking: bool,
}

impl Default for ComputerThinkTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
            thinking: false,
        }
    }
}

fn computer_turn(
    time: Res<Time>,
    mut think_timer: ResMut<ComputerThinkTimer>,
    settings: Res<GameSettings>,
    game_state: Res<GameState>,
    pieces: Query<(Entity, &Piece, &Children)>,
    pieces_for_validation: Query<(&Piece, &Children)>,
    moving_pieces: Query<&super::animation::MovingPiece>,
    mut move_events: EventWriter<MoveEvent>,
) {
    if settings.mode != GameMode::VsComputer {
        return;
    }

    if game_state.game_over {
        think_timer.thinking = false;
        return;
    }

    if game_state.current_turn != PieceColor::Black {
        think_timer.thinking = false;
        return;
    }

    if !moving_pieces.is_empty() {
        think_timer.thinking = false;
        return;
    }

    if !think_timer.thinking {
        think_timer.thinking = true;
        think_timer.timer.reset();
        return;
    }

    think_timer.timer.tick(time.delta());

    if !think_timer.timer.finished() {
        return;
    }

    let mut valid_moves: Vec<(Entity, usize)> = Vec::new();

    for (entity, piece, _) in &pieces {
        if piece.color == PieceColor::Black {
            let moves = get_valid_moves(piece, &pieces_for_validation);
            for target in moves {
                valid_moves.push((entity, target));
            }
        }
    }

    if let Some((entity, target)) = valid_moves.choose(&mut rand::rng()) {
        move_events.write(MoveEvent {
            piece_entity: *entity,
            target_node: *target,
        });
    }

    think_timer.thinking = false;
}
