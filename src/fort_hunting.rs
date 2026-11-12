//! Part 2: Fort, Hunting & Shooting.
//!
//! Every function below is a skeleton: it compiles and returns a harmless default so the
//! other parts can build against it. Replace the body with your own code. Keep the
//! signature unless your team and instructor agree to change it.

use crate::types::{GameState, TurnChoice};
use std::time::Instant;


/// Shows the right turn menu (fort or no fort), settles a valid, affordable choice, flips
/// `fort_available_next_turn`, and returns the choice.
///
/// Skeleton returns `TurnChoice::Continue`.
pub fn turn_menu(state: &mut GameState) -> TurnChoice {
    TurnChoice::Continue
}

/// Fort purchases at the worse 2:3 exchange rate; stopping costs 45 miles.
///
/// Skeleton does nothing.
pub fn fort_purchases(state: &mut GameState) {}

/// Hunting: bullet check, 45 miles, shooting challenge, then `resolve_hunt_outcome`.
///
/// Skeleton does nothing.
pub fn go_hunting(state: &mut GameState)
{
    if state.bullets < 40.0 {
        println!("TOUGH---YOU NEED MORE BULLETS TO GO HUNTING");
        return;
    }

    state.mileage -= 45.0;
    let score = shooting_challenge(state.skill_level);
    resolve_hunt_outcome(score, state);
}


/// The timed "type the word" challenge. Returns the response score: lower is faster, and
/// 9 means the wrong word was typed. Also called from Parts 3 and 4.
///
/// Skeleton returns 2.0 (a middling response) without prompting.
pub fn shooting_challenge(skill_level: f64) -> f64 {
    let words = ["BANG", "BLAM", "POW", "WHAM"];
    let index = ((crate::rng::rnd() * words.len() as f64) as usize).min(words.len() - 1);
    let target = words[index];

    println!("TYPE {}", target);
    let start = Instant::now();
    let response = crate::io::ask("");
    let elapsed = start.elapsed().as_secs_f64();

    if response != target {
        9.0
    } else {
        (elapsed - (skill_level - 1.0)).max(0.0)
    }

}

/// Turns a response score into a hunt result: food gained, bullets spent, or a miss.
///
/// Skeleton does nothing.
pub fn resolve_hunt_outcome(score: f64, state: &mut GameState) {}
