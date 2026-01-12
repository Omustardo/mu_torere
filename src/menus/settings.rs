//! The settings menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    games::mu_torere::GameSettings,
    menus::Menu,
    screens::{is_playing, Screen},
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
    );

    app.add_systems(
        Update,
        update_instant_animation_label.run_if(in_state(Menu::Settings)),
    );
}

fn spawn_settings_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Settings Menu"),
        GlobalZIndex(2),
        StateScoped::new(Menu::Settings),
        children![
            widget::header("Settings"),
            settings_grid(),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn settings_grid() -> impl Bundle {
    (
        Name::new("Settings Grid"),
        Node {
            display: Display::Grid,
            row_gap: Val::Px(10.0),
            column_gap: Val::Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            (
                widget::label("Instant Animation"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            instant_animation_widget(),
        ],
    )
}

fn instant_animation_widget() -> impl Bundle {
    (
        Name::new("Instant Animation Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("<", toggle_instant_animation),
            (
                Name::new("Current Setting"),
                Node {
                    padding: UiRect::horizontal(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    min_width: Val::Px(60.0),
                    ..default()
                },
                children![(widget::label(""), InstantAnimationLabel)],
            ),
            widget::button_small(">", toggle_instant_animation),
        ],
    )
}

fn toggle_instant_animation(_: On<Pointer<Click>>, mut settings: ResMut<GameSettings>) {
    settings.instant_animation = !settings.instant_animation;
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct InstantAnimationLabel;

fn update_instant_animation_label(
    settings: Res<GameSettings>,
    mut label: Single<&mut Text, With<InstantAnimationLabel>>,
) {
    label.0 = if settings.instant_animation {
        "On".to_string()
    } else {
        "Off".to_string()
    };
}

fn go_back_on_click(
    _: On<Pointer<Click>>,
    screen: Res<State<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    // If we're playing a game, go back to pause menu; otherwise go to game select
    next_menu.set(if is_playing(screen) {
        Menu::Pause
    } else {
        Menu::GameSelect
    });
}

fn go_back(screen: Res<State<Screen>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(if is_playing(screen) {
        Menu::Pause
    } else {
        Menu::GameSelect
    });
}
