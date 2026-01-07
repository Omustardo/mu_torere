//! Board representation and rendering for Mu Torere.

use bevy::prelude::*;
use std::f32::consts::PI;

use crate::screens::Screen;

use super::state::{GameState, PieceColor};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_board);
    app.add_systems(
        Update,
        update_piece_colors.run_if(in_state(Screen::Gameplay)),
    );
}

pub const CENTER_INDEX: usize = 8;
pub const OUTER_RADIUS: f32 = 200.0;
pub const PIECE_RADIUS: f32 = 25.0;
pub const LINE_WIDTH: f32 = 4.0;

#[derive(Component)]
pub struct BoardNode {
    pub index: usize,
}

#[derive(Component)]
pub struct Piece {
    pub color: PieceColor,
    pub node_index: usize,
}

#[derive(Component)]
pub struct PieceVisual;

#[derive(Component)]
pub struct HighlightRing;

fn get_node_position(index: usize) -> Vec2 {
    if index == CENTER_INDEX {
        Vec2::ZERO
    } else {
        let angle = (index as f32) * (2.0 * PI / 8.0) - PI / 2.0;
        Vec2::new(angle.cos() * OUTER_RADIUS, angle.sin() * OUTER_RADIUS)
    }
}

pub fn get_adjacencies(index: usize) -> Vec<usize> {
    if index == CENTER_INDEX {
        (0..8).collect()
    } else {
        let prev = (index + 7) % 8;
        let next = (index + 1) % 8;
        vec![prev, next, CENTER_INDEX]
    }
}

fn spawn_board(mut commands: Commands) {
    commands.spawn((
        Name::new("Board"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![board_lines(), board_nodes_and_pieces(),],
    ));
}

fn board_lines() -> impl Bundle {
    (
        Name::new("Board Lines"),
        Transform::default(),
        Visibility::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            let line_color = Color::srgb(0.4, 0.4, 0.4);

            for i in 0..8 {
                let pos = get_node_position(i);
                let next_pos = get_node_position((i + 1) % 8);

                let mid = (pos + next_pos) / 2.0;
                let diff = next_pos - pos;
                let length = diff.length();
                let angle = diff.y.atan2(diff.x);

                parent.spawn((
                    Name::new(format!("Outer Line {i}")),
                    Sprite {
                        color: line_color,
                        custom_size: Some(Vec2::new(length, LINE_WIDTH)),
                        ..default()
                    },
                    Transform::from_xyz(mid.x, mid.y, 0.0)
                        .with_rotation(Quat::from_rotation_z(angle)),
                ));

                let center = Vec2::ZERO;
                let mid_to_center = (pos + center) / 2.0;
                let diff_to_center = center - pos;
                let length_to_center = diff_to_center.length();
                let angle_to_center = diff_to_center.y.atan2(diff_to_center.x);

                parent.spawn((
                    Name::new(format!("Radial Line {i}")),
                    Sprite {
                        color: line_color,
                        custom_size: Some(Vec2::new(length_to_center, LINE_WIDTH)),
                        ..default()
                    },
                    Transform::from_xyz(mid_to_center.x, mid_to_center.y, 0.0)
                        .with_rotation(Quat::from_rotation_z(angle_to_center)),
                ));
            }
        })),
    )
}

fn board_nodes_and_pieces() -> impl Bundle {
    (
        Name::new("Nodes and Pieces"),
        Transform::default(),
        Visibility::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            for i in 0..9 {
                let pos = get_node_position(i);

                parent.spawn((
                    Name::new(format!("Node {i}")),
                    BoardNode { index: i },
                    Sprite {
                        color: Color::srgb(0.2, 0.2, 0.2),
                        custom_size: Some(Vec2::splat(PIECE_RADIUS * 2.0 + 10.0)),
                        ..default()
                    },
                    Transform::from_xyz(pos.x, pos.y, 1.0),
                ));
            }

            // Initial piece placement: White on nodes 0-3, Black on nodes 4-7
            // Center (node 8) starts empty
            for i in 0..8 {
                let color = if i < 4 {
                    PieceColor::White
                } else {
                    PieceColor::Black
                };
                let pos = get_node_position(i);

                parent.spawn((
                    Name::new(format!("Piece {i}")),
                    Piece {
                        color,
                        node_index: i,
                    },
                    Transform::from_xyz(pos.x, pos.y, 2.0),
                    Visibility::default(),
                    children![highlight_ring(), piece_visual(color),],
                ));
            }
        })),
    )
}

fn highlight_ring() -> impl Bundle {
    (
        Name::new("Highlight Ring"),
        HighlightRing,
        Sprite {
            color: Color::srgba(1.0, 1.0, 0.0, 0.0),
            custom_size: Some(Vec2::splat(PIECE_RADIUS * 2.0 + 16.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -0.1),
    )
}

fn piece_visual(color: PieceColor) -> impl Bundle {
    let visual_color = match color {
        PieceColor::White => Color::srgb(0.95, 0.95, 0.95),
        PieceColor::Black => Color::srgb(0.1, 0.1, 0.1),
    };

    (
        Name::new("Piece Visual"),
        PieceVisual,
        Sprite {
            color: visual_color,
            custom_size: Some(Vec2::splat(PIECE_RADIUS * 2.0)),
            ..default()
        },
        Transform::default(),
    )
}

fn update_piece_colors(
    game_state: Res<GameState>,
    pieces: Query<(&Piece, &Children)>,
    mut highlights: Query<&mut Sprite, With<HighlightRing>>,
) {
    for (piece, children) in &pieces {
        let can_move = !game_state.game_over
            && piece.color == game_state.current_turn
            && can_piece_move(piece, &pieces);

        for child in children.iter() {
            if let Ok(mut sprite) = highlights.get_mut(child) {
                sprite.color = if can_move {
                    Color::srgba(1.0, 1.0, 0.0, 0.8)
                } else {
                    Color::srgba(1.0, 1.0, 0.0, 0.0)
                };
            }
        }
    }
}

pub fn can_piece_move(piece: &Piece, pieces: &Query<(&Piece, &Children)>) -> bool {
    let adjacencies = get_adjacencies(piece.node_index);

    for adj_index in adjacencies {
        if is_node_empty(adj_index, pieces) {
            if adj_index == CENTER_INDEX || piece.node_index == CENTER_INDEX {
                if is_adjacent_to_opponent(piece, pieces) {
                    return true;
                }
            } else {
                return true;
            }
        }
    }
    false
}

pub fn is_node_empty(node_index: usize, pieces: &Query<(&Piece, &Children)>) -> bool {
    for (piece, _) in pieces.iter() {
        if piece.node_index == node_index {
            return false;
        }
    }
    true
}

pub fn is_adjacent_to_opponent(piece: &Piece, pieces: &Query<(&Piece, &Children)>) -> bool {
    let adjacencies = get_adjacencies(piece.node_index);
    for (other_piece, _) in pieces.iter() {
        if other_piece.color != piece.color && adjacencies.contains(&other_piece.node_index) {
            return true;
        }
    }
    false
}

pub fn get_valid_moves(piece: &Piece, pieces: &Query<(&Piece, &Children)>) -> Vec<usize> {
    let mut moves = Vec::new();
    let adjacencies = get_adjacencies(piece.node_index);

    for adj_index in adjacencies {
        if is_node_empty(adj_index, pieces) {
            if adj_index == CENTER_INDEX || piece.node_index == CENTER_INDEX {
                if is_adjacent_to_opponent(piece, pieces) {
                    moves.push(adj_index);
                }
            } else {
                moves.push(adj_index);
            }
        }
    }
    moves
}

pub fn has_any_valid_moves(color: PieceColor, pieces: &Query<(&Piece, &Children)>) -> bool {
    for (piece, _) in pieces.iter() {
        if piece.color == color && can_piece_move(piece, pieces) {
            return true;
        }
    }
    false
}

pub fn node_position(index: usize) -> Vec2 {
    get_node_position(index)
}
