//! UI for the Mental Math game.

use bevy::prelude::*;

use super::{
    is_playing_mental_math,
    problem::{generate_new_problem, MathProblem},
};
use crate::{
    screens::{ActiveGame, Screen},
    theme::{palette::*, widget},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Playing(ActiveGame::MentalMath)),
        spawn_game_ui,
    );
    app.add_systems(
        Update,
        (
            update_problem_display,
            update_result_display,
            update_choice_buttons,
            on_choice_click,
        )
            .run_if(is_playing_mental_math),
    );
}

/// Marker for the problem display text
#[derive(Component)]
struct ProblemDisplay;

/// Marker for the result display (shows correct/incorrect and the answer)
#[derive(Component)]
struct ResultDisplay;

/// Marker for choice buttons, with the choice index
#[derive(Component)]
struct ChoiceButton(usize);

/// Marker for the choice button text
#[derive(Component)]
struct ChoiceButtonText(usize);

/// Marker for the "New Problem" button
#[derive(Component)]
struct NewProblemButton;

fn spawn_game_ui(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Mental Math UI"),
        StateScoped(Screen::Playing(ActiveGame::MentalMath)),
        children![
            // Problem display with math styling
            (
                Name::new("Problem Container"),
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                children![
                    (
                        ProblemDisplay,
                        Name::new("Problem Text"),
                        Text::new(""),
                        TextFont::from_font_size(56.0),
                        TextColor(HEADER_TEXT),
                    ),
                    (
                        Name::new("Equals Sign"),
                        Text::new(" = ?"),
                        TextFont::from_font_size(56.0),
                        TextColor(LABEL_TEXT),
                    ),
                ],
            ),
            // Result display (hidden until answered)
            (
                ResultDisplay,
                Name::new("Result Text"),
                Text::new(""),
                TextFont::from_font_size(32.0),
                TextColor(LABEL_TEXT),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ),
            // Choice buttons container
            (
                Name::new("Choices Container"),
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(20.0),
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
                children![
                    choice_button(0),
                    choice_button(1),
                    choice_button(2),
                    choice_button(3),
                ],
            ),
            // New Problem button
            widget::button("New Problem", on_new_problem_click),
        ],
    ));
}

/// Create a choice button for the given index
fn choice_button(index: usize) -> impl Bundle {
    (
        Name::new(format!("Choice Button {}", index)),
        ChoiceButton(index),
        Button,
        Node {
            width: Val::Px(120.0),
            height: Val::Px(80.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(10.0)),
        BackgroundColor(BUTTON_BACKGROUND),
        children![(
            ChoiceButtonText(index),
            Name::new("Choice Text"),
            Text::new(""),
            TextFont::from_font_size(36.0),
            TextColor(BUTTON_TEXT),
            Pickable::IGNORE,
        )],
    )
}

/// Update the problem display when the problem changes
fn update_problem_display(
    problem: Res<MathProblem>,
    mut query: Query<&mut Text, With<ProblemDisplay>>,
) {
    if problem.is_changed() {
        for mut text in &mut query {
            **text = problem.display.clone();
        }
    }
}

/// Update the result display after answering
fn update_result_display(
    problem: Res<MathProblem>,
    mut query: Query<(&mut Text, &mut TextColor), With<ResultDisplay>>,
) {
    if problem.is_changed() {
        for (mut text, mut color) in &mut query {
            if problem.answered {
                if problem.is_correct() {
                    **text = "Correct!".to_string();
                    *color = TextColor(Color::srgb(0.4, 0.9, 0.4)); // Green
                } else {
                    **text = format!("The answer was {}", problem.answer);
                    *color = TextColor(Color::srgb(0.9, 0.4, 0.4)); // Red
                }
            } else {
                **text = String::new();
            }
        }
    }
}

/// Update choice button text and colors
fn update_choice_buttons(
    problem: Res<MathProblem>,
    mut text_query: Query<(&ChoiceButtonText, &mut Text)>,
    mut button_query: Query<(&ChoiceButton, &mut BackgroundColor, &Interaction)>,
) {
    // Update button text
    if problem.is_changed() {
        for (choice_text, mut text) in &mut text_query {
            if let Some(&value) = problem.choices.get(choice_text.0) {
                **text = value.to_string();
            }
        }
    }

    // Update button colors based on state
    for (choice_button, mut bg_color, interaction) in &mut button_query {
        let index = choice_button.0;

        if problem.answered {
            // After answering, show correct/incorrect colors
            if index == problem.correct_index {
                // Correct answer - green
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.6, 0.2));
            } else if Some(index) == problem.player_choice {
                // Player's wrong choice - red
                *bg_color = BackgroundColor(Color::srgb(0.7, 0.2, 0.2));
            } else {
                // Other choices - dimmed
                *bg_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.4));
            }
        } else {
            // Normal interaction colors
            match *interaction {
                Interaction::Pressed => {
                    *bg_color = BackgroundColor(BUTTON_PRESSED_BACKGROUND);
                }
                Interaction::Hovered => {
                    *bg_color = BackgroundColor(BUTTON_HOVERED_BACKGROUND);
                }
                Interaction::None => {
                    *bg_color = BackgroundColor(BUTTON_BACKGROUND);
                }
            }
        }
    }
}

/// Handle clicking a choice button
pub fn on_choice_click(
    mut interaction_query: Query<
        (&Interaction, &ChoiceButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut problem: ResMut<MathProblem>,
) {
    for (interaction, choice_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed && !problem.answered {
            problem.answered = true;
            problem.player_choice = Some(choice_button.0);
        }
    }
}

/// Handle clicking the "New Problem" button
fn on_new_problem_click(_: On<Pointer<Click>>, problem: ResMut<MathProblem>) {
    generate_new_problem(problem);
}
