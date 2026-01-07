//! Piece movement animation for Mu Torere.

use bevy::prelude::*;

use crate::screens::Screen;

use super::{
    board::{Piece, node_position},
    state::{GameOverEvent, GameSettings, GameState, PieceColor, TurnChangeEvent},
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<MoveEvent>();
    app.add_systems(
        Update,
        (handle_move_events, animate_pieces, check_animation_complete)
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Event)]
pub struct MoveEvent {
    pub piece_entity: Entity,
    pub target_node: usize,
}

#[derive(Component)]
pub struct MovingPiece {
    pub start_pos: Vec2,
    pub end_pos: Vec2,
    pub progress: f32,
    pub target_node: usize,
}

const ANIMATION_SPEED: f32 = 5.0;

fn handle_move_events(
    mut commands: Commands,
    mut move_events: EventReader<MoveEvent>,
    mut pieces: Query<(&mut Piece, &Transform)>,
    settings: Res<GameSettings>,
) {
    for event in move_events.read() {
        if let Ok((mut piece, transform)) = pieces.get_mut(event.piece_entity) {
            let start_pos = transform.translation.truncate();
            let end_pos = node_position(event.target_node);

            piece.node_index = event.target_node;

            if settings.instant_animation {
                commands
                    .entity(event.piece_entity)
                    .insert(Transform::from_xyz(end_pos.x, end_pos.y, 2.0));
                commands.entity(event.piece_entity).insert(MovingPiece {
                    start_pos: end_pos,
                    end_pos,
                    progress: 1.0,
                    target_node: event.target_node,
                });
            } else {
                commands.entity(event.piece_entity).insert(MovingPiece {
                    start_pos,
                    end_pos,
                    progress: 0.0,
                    target_node: event.target_node,
                });
            }
        }
    }
}

fn animate_pieces(
    time: Res<Time>,
    mut pieces: Query<(&mut Transform, &mut MovingPiece)>,
    settings: Res<GameSettings>,
) {
    for (mut transform, mut moving) in &mut pieces {
        if moving.progress < 1.0 {
            let speed = if settings.instant_animation {
                1000.0
            } else {
                ANIMATION_SPEED
            };
            moving.progress = (moving.progress + time.delta_secs() * speed).min(1.0);
            let t = ease_out_quad(moving.progress);
            let pos = moving.start_pos.lerp(moving.end_pos, t);
            transform.translation = pos.extend(2.0);
        }
    }
}

fn ease_out_quad(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

fn check_animation_complete(
    mut commands: Commands,
    pieces_moving: Query<(Entity, &Piece, &MovingPiece)>,
    pieces_all: Query<(&Piece, &Children)>,
    mut game_state: ResMut<GameState>,
    mut turn_events: EventWriter<TurnChangeEvent>,
    mut game_over_events: EventWriter<GameOverEvent>,
) {
    for (entity, piece, moving) in &pieces_moving {
        if moving.progress >= 1.0 {
            commands.entity(entity).remove::<MovingPiece>();

            let next_turn = piece.color.opposite();

            if !super::board::has_any_valid_moves(next_turn, &pieces_all) {
                game_state.game_over = true;
                game_state.winner = Some(piece.color);
                game_over_events.write(GameOverEvent {
                    winner: piece.color,
                });
            } else {
                game_state.current_turn = next_turn;
                turn_events.write(TurnChangeEvent {
                    new_turn: next_turn,
                });
            }
        }
    }
}

pub fn is_any_piece_moving(pieces: &Query<(Entity, &Piece, &MovingPiece)>) -> bool {
    !pieces.is_empty()
}

pub fn is_piece_moving(
    entity: Entity,
    pieces: &Query<(Entity, &Piece, &MovingPiece), Without<PieceColor>>,
) -> bool {
    pieces.get(entity).is_ok()
}
