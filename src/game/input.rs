//! Player input handling for Mu Torere.

use bevy::{prelude::*, window::PrimaryWindow};

use crate::screens::Screen;

use super::{
    animation::MoveEvent,
    board::{get_valid_moves, node_position, Piece, PIECE_RADIUS},
    state::{GameMode, GameSettings, GameState, PieceColor},
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<SelectedPiece>();
    app.add_systems(
        Update,
        (handle_click, clear_selection_on_game_over).run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Resource, Default)]
pub struct SelectedPiece {
    pub entity: Option<Entity>,
}

fn handle_click(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut selected: ResMut<SelectedPiece>,
    pieces: Query<(Entity, &Piece, &Transform, &Children)>,
    pieces_for_validation: Query<(&Piece, &Children)>,
    game_state: Res<GameState>,
    settings: Res<GameSettings>,
    moving_pieces: Query<&super::animation::MovingPiece>,
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

    if let Some(selected_entity) = selected.entity {
        if let Ok((_, piece, _, _)) = pieces.get(selected_entity) {
            let valid_moves = get_valid_moves(piece, &pieces_for_validation);

            for target_node in &valid_moves {
                let node_pos = node_position(*target_node);
                if world_pos.distance(node_pos) < PIECE_RADIUS + 15.0 {
                    move_events.write(MoveEvent {
                        piece_entity: selected_entity,
                        target_node: *target_node,
                    });
                    selected.entity = None;
                    return;
                }
            }
        }
    }

    for (entity, piece, transform, _) in &pieces {
        let piece_pos = transform.translation.truncate();
        if world_pos.distance(piece_pos) < PIECE_RADIUS + 5.0 {
            if piece.color == current_turn {
                let valid_moves = get_valid_moves(piece, &pieces_for_validation);
                if !valid_moves.is_empty() {
                    selected.entity = Some(entity);
                    return;
                }
            }
        }
    }

    selected.entity = None;
}

fn clear_selection_on_game_over(game_state: Res<GameState>, mut selected: ResMut<SelectedPiece>) {
    if game_state.game_over {
        selected.entity = None;
    }
}
