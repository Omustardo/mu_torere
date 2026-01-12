//! A loading screen during which game assets are loaded if necessary.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

use crate::{
    asset_tracking::ResourceHandles,
    screens::{is_loading, ActiveGame, Screen},
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Loading(ActiveGame::MuTorere)),
        spawn_loading_screen,
    );
    // Add OnEnter for other games as they're added

    app.add_systems(
        Update,
        check_loading_complete.run_if(is_loading.and(all_assets_loaded)),
    );
}

#[derive(Component)]
struct LoadingScreen;

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        LoadingScreen,
        widget::ui_root("Loading Screen"),
        StateScoped::new(Screen::Loading(ActiveGame::MuTorere)),
        children![widget::label("Loading...")],
    ));
}

fn check_loading_complete(screen: Res<State<Screen>>, mut next_screen: ResMut<NextState<Screen>>) {
    // Transition from Loading(game) to Playing(game)
    if let Screen::Loading(game) = screen.get() {
        next_screen.set(Screen::Playing(*game));
    }
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}
