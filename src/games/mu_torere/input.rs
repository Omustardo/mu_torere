//! Player input handling for Mu Torere.

use bevy::{prelude::*, window::PrimaryWindow};

use crate::PausableSystems;

use super::{
    animation::{MoveEvent, MovingPiece},
    board::{get_valid_moves, Piece, PIECE_RADIUS},
    is_playing_mu_torere,
    state::{GameMode, GameSettings, GameState, PieceColor},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        handle_click
            .run_if(is_playing_mu_torere)
            .in_set(PausableSystems),
    );
}

fn handle_click(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    pieces: Query<(Entity, &Piece, &Transform)>,
    pieces_for_validation: Query<(&Piece, &Children)>,
    game_state: Res<GameState>,
    settings: Res<GameSettings>,
    moving_pieces: Query<&MovingPiece>,
    mut move_events: MessageWriter<MoveEvent>,
) {
    if game_state.game_over {
        return;
    }

    if !moving_pieces.is_empty() {
        return;
    }

    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };

    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    let current_turn = game_state.current_turn;
    let is_player_turn = match settings.mode {
        GameMode::VsPlayer => true,
        GameMode::VsComputer => current_turn == PieceColor::White,
    };

    if !is_player_turn {
        return;
    }

    // Find clicked piece and move it if it has a valid move
    for (entity, piece, transform) in &pieces {
        let piece_pos = transform.translation.truncate();
        if world_pos.distance(piece_pos) < PIECE_RADIUS + 5.0 {
            if piece.color == current_turn {
                let valid_moves = get_valid_moves(piece, &pieces_for_validation);
                if let Some(&target_node) = valid_moves.first() {
                    move_events.write(MoveEvent {
                        piece_entity: entity,
                        target_node,
                    });
                    return;
                }
            }
        }
    }
}
