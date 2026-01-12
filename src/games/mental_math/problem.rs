//! Math problem generation using postfix notation.

use bevy::prelude::*;
use rand::prelude::*;

use super::is_playing_mental_math;
use crate::screens::{ActiveGame, Screen};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MathProblem>();
    app.add_systems(
        OnEnter(Screen::Playing(ActiveGame::MentalMath)),
        generate_new_problem,
    );
}

/// The current math problem state
#[derive(Resource, Default)]
pub struct MathProblem {
    /// The problem displayed as infix notation
    pub display: String,
    /// The correct answer
    pub answer: i32,
    /// Multiple choice options (includes the correct answer)
    pub choices: Vec<i32>,
    /// Index of the correct answer in choices
    pub correct_index: usize,
    /// Whether the player has answered
    pub answered: bool,
    /// Index of the player's choice (if answered)
    pub player_choice: Option<usize>,
}

impl MathProblem {
    /// Check if the player's answer was correct
    pub fn is_correct(&self) -> bool {
        self.player_choice == Some(self.correct_index)
    }
}

/// Operators for math expressions
#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn symbol(&self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Subtract => "\u{2212}", // Unicode minus sign
            Operator::Multiply => "\u{00D7}", // Unicode multiplication sign
            Operator::Divide => "\u{00F7}",   // Unicode division sign
        }
    }

    fn precedence(&self) -> u8 {
        match self {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 2,
        }
    }

    fn apply(&self, left: i32, right: i32) -> Option<i32> {
        match self {
            Operator::Add => left.checked_add(right),
            Operator::Subtract => left.checked_sub(right),
            Operator::Multiply => left.checked_mul(right),
            Operator::Divide => {
                if right != 0 && left % right == 0 {
                    Some(left / right)
                } else {
                    None
                }
            }
        }
    }
}

/// A token in a postfix expression
#[derive(Clone, Debug)]
enum Token {
    Number(i32),
    Op(Operator),
}

/// A math expression built using postfix notation
struct PostfixExpression {
    tokens: Vec<Token>,
}

impl PostfixExpression {
    /// Generate a random expression with the given number of operations
    fn generate(rng: &mut impl Rng, num_operations: usize) -> Option<Self> {
        // Start with a number on the stack
        let mut tokens = vec![Token::Number(rng.gen_range(2..=12))];
        let mut stack_size = 1;

        for i in 0..num_operations {
            // Always push a number first
            let num = rng.gen_range(2..=12);
            tokens.push(Token::Number(num));
            stack_size += 1;

            // Choose an operator
            let op = if i == num_operations - 1 {
                // Last operation - prefer simpler ops
                match rng.gen_range(0..4) {
                    0 => Operator::Add,
                    1 => Operator::Subtract,
                    2 => Operator::Multiply,
                    _ => Operator::Add, // Avoid division on last op to keep things simpler
                }
            } else {
                match rng.gen_range(0..4) {
                    0 => Operator::Add,
                    1 => Operator::Subtract,
                    2 => Operator::Multiply,
                    _ => Operator::Divide,
                }
            };

            tokens.push(Token::Op(op));
            stack_size -= 1; // Op consumes 2, produces 1
        }

        Some(Self { tokens })
    }

    /// Evaluate the postfix expression
    fn evaluate(&self) -> Option<i32> {
        let mut stack = Vec::new();

        for token in &self.tokens {
            match token {
                Token::Number(n) => stack.push(*n),
                Token::Op(op) => {
                    let right = stack.pop()?;
                    let left = stack.pop()?;
                    let result = op.apply(left, right)?;
                    stack.push(result);
                }
            }
        }

        if stack.len() == 1 {
            Some(stack[0])
        } else {
            None
        }
    }

    /// Convert to infix notation with proper parentheses
    fn to_infix(&self) -> Option<String> {
        #[derive(Clone)]
        struct InfixPart {
            text: String,
            precedence: u8,
        }

        let mut stack: Vec<InfixPart> = Vec::new();

        for token in &self.tokens {
            match token {
                Token::Number(n) => {
                    stack.push(InfixPart {
                        text: n.to_string(),
                        precedence: u8::MAX,
                    });
                }
                Token::Op(op) => {
                    let right = stack.pop()?;
                    let left = stack.pop()?;

                    let op_prec = op.precedence();

                    // Add parentheses if needed based on precedence
                    let left_text = if left.precedence < op_prec {
                        format!("({})", left.text)
                    } else {
                        left.text
                    };

                    // Right side needs parens if lower precedence, or if same precedence
                    // for subtraction/division (left-associative)
                    let right_text = if right.precedence < op_prec
                        || (right.precedence == op_prec
                            && matches!(op, Operator::Subtract | Operator::Divide))
                    {
                        format!("({})", right.text)
                    } else {
                        right.text
                    };

                    stack.push(InfixPart {
                        text: format!("{} {} {}", left_text, op.symbol(), right_text),
                        precedence: op_prec,
                    });
                }
            }
        }

        if stack.len() == 1 {
            Some(stack[0].text.clone())
        } else {
            None
        }
    }
}

/// Generate distractors (wrong answers) that are plausible
fn generate_distractors(rng: &mut impl Rng, correct: i32, count: usize) -> Vec<i32> {
    let mut distractors = Vec::new();

    // Generate plausible wrong answers
    let offsets = [
        -10, -5, -3, -2, -1, 1, 2, 3, 5, 10, // Simple offsets
    ];

    // Try to add diverse distractors
    let mut attempts = 0;
    while distractors.len() < count && attempts < 100 {
        attempts += 1;

        let distractor = if rng.gen_bool(0.7) {
            // Use an offset from the correct answer
            let offset = offsets[rng.gen_range(0..offsets.len())];
            correct + offset
        } else {
            // Generate a random number in a reasonable range
            let range = correct.abs().max(10);
            rng.gen_range((correct - range)..(correct + range + 1))
        };

        // Don't add duplicates or the correct answer
        if distractor != correct && !distractors.contains(&distractor) {
            distractors.push(distractor);
        }
    }

    // Fill with simple offsets if needed
    let mut offset = 1;
    while distractors.len() < count {
        if !distractors.contains(&(correct + offset)) {
            distractors.push(correct + offset);
        } else if !distractors.contains(&(correct - offset)) {
            distractors.push(correct - offset);
        }
        offset += 1;
    }

    distractors
}

/// Generate a new math problem
pub fn generate_new_problem(mut problem: ResMut<MathProblem>) {
    let mut rng = rand::thread_rng();

    // Try to generate a valid problem
    for _ in 0..100 {
        // Randomly choose 1-2 operations for manageable mental math
        let num_ops = rng.gen_range(1..=2);

        if let Some(expr) = PostfixExpression::generate(&mut rng, num_ops) {
            if let (Some(answer), Some(display)) = (expr.evaluate(), expr.to_infix()) {
                // Skip problems with answers that are too large or negative
                if answer.abs() > 1000 {
                    continue;
                }

                // Generate 3 distractors for 4 total choices
                let distractors = generate_distractors(&mut rng, answer, 3);

                // Create choices and shuffle
                let mut choices: Vec<i32> = distractors;
                choices.push(answer);
                choices.shuffle(&mut rng);

                let correct_index = choices.iter().position(|&x| x == answer).unwrap();

                *problem = MathProblem {
                    display,
                    answer,
                    choices,
                    correct_index,
                    answered: false,
                    player_choice: None,
                };

                return;
            }
        }
    }

    // Fallback to a simple problem if generation fails
    let a = rng.gen_range(2..=12);
    let b = rng.gen_range(2..=12);
    let answer = a + b;
    let display = format!("{} + {}", a, b);
    let distractors = generate_distractors(&mut rng, answer, 3);
    let mut choices: Vec<i32> = distractors;
    choices.push(answer);
    choices.shuffle(&mut rng);
    let correct_index = choices.iter().position(|&x| x == answer).unwrap();

    *problem = MathProblem {
        display,
        answer,
        choices,
        correct_index,
        answered: false,
        player_choice: None,
    };
}
