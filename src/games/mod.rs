//! Individual game implementations.
//! Each game is a self-contained module with its own plugin.

pub mod mental_math;
pub mod mu_torere;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((mental_math::plugin, mu_torere::plugin));
}
