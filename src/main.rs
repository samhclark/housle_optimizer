use std::process::exit;

use rand::{thread_rng, Rng};

struct State {
    guess: i32,
    lower: i32,
    upper: i32,
}

enum Outcome {
    TooHigh,
    TooLow,
}

fn main() {
    let mut rng = thread_rng();
    let house_value = rng.gen_range(0..=50_000_000);

    // Round 1
    let round_1 = State {
        guess: 6_000_000,
        lower: 0,
        upper: 50_000_000,
    };
    let outcome_1 = run(round_1.guess, house_value);

    // Round 2
    let round_2 = next_state(round_1, outcome_1);
    let outcome_2 = run(round_2.guess, house_value);

    // Round 3
    let round_3 = next_state(round_2, outcome_2);
    let outcome_3 = run(round_3.guess, house_value);

    // Round 4
    let round_4 = next_state(round_3, outcome_3);
    let outcome_4 = run(round_4.guess, house_value);

    // Round 5
    let round_5 = next_state(round_4, outcome_4);
    let outcome_5 = run(round_5.guess, house_value);

    // Round 6
    let round_6 = next_state(round_5, outcome_5);
    let outcome_6 = run(round_6.guess, house_value);

    match outcome_6 {
        Outcome::TooHigh => println!(
            "Too high. Last guess: {}. Value: {}. ({}...{})",
            round_6.guess,
            house_value,
            (house_value as i64 * 95 / 100) as i32,
            (house_value as i64 * 105 / 100) as i32
        ),
        Outcome::TooLow => println!(
            "Too low. Last guess: {}. Value: {}. ({}...{})",
            round_6.guess,
            house_value,
            house_value * 95 / 100,
            house_value * 105 / 100
        ),
    }
}

fn run(guess: i32, answer: i32) -> Outcome {
    let lower_bound: i32 = (answer as i64 * 95 / 100) as i32;
    let upper_bound = (answer as i64 * 105 / 100) as i32;
    if guess >= lower_bound && guess <= upper_bound {
        println!("You win! Guess: {guess}. Value: {answer}. ({lower_bound}...{upper_bound})");
        exit(0)
    } else if guess < lower_bound {
        Outcome::TooLow
    } else {
        Outcome::TooHigh
    }
}

fn next_state(prev_state: State, prev_outcome: Outcome) -> State {
    match prev_outcome {
        Outcome::TooHigh => State {
            guess: midpoint(prev_state.lower, prev_state.guess),
            lower: prev_state.lower,
            upper: prev_state.guess,
        },
        Outcome::TooLow => State {
            guess: midpoint(prev_state.guess, prev_state.upper),
            lower: prev_state.guess,
            upper: prev_state.upper,
        },
    }
}

fn midpoint(lower: i32, upper: i32) -> i32 {
    lower + upper / 2
}
