//! Individual game implementations.
//! Each game is a self-contained module with its own plugin.

pub mod mu_torere;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(mu_torere::plugin);
}
