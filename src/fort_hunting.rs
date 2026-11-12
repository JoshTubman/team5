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
fn evaluate_shooting_response(target: &str, response: &str, elapsed: f64, skill_level: f64) -> f64 {
    if response != target {
        9.0
    } else {
        (elapsed - (skill_level - 1.0)).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_state(bullets: f64, mileage: f64, skill_level: f64) -> GameState {
        GameState {
            skill_level,
            bullets,
            food: 200.0,
            mileage,
            cash: 300.0,
            clothing: 100.0,
            misc_supplies: 50.0,
            ..GameState::default()

        }
    }

    #[test]
    fn test_shooting_normal_inputs() {
        let baseline = evaluate_shooting_response("BANG", "BANG", 0.85, 1.0);
        println!("Normal exact match (skill 1.0, 0.85s) -> score: {}", baseline);
        assert!((baseline - 0.85).abs() < 1e-6);

        let skilled = evaluate_shooting_response("POW", "POW", 2.5, 2.0);
        println!("Normal exact match (skill 2.0, 2.5s) -> score: {}", skilled);
        assert!((skilled - 1.5).abs() < 1e-6);
    }

    #[test]
    fn test_shooting_and_hunting_boundary_inputs() {
        // Boundary: 39.0 bullets (1 below 40.0 minimum) must block hunting and preserve state
        let mut state_39 = sample_state(39.0, 500.0, 2.0);
        go_hunting(&mut state_39);
        println!(
            "Boundary 39.0 bullets -> mileage: {}, bullets: {}",
            state_39.mileage, state_39.bullets
        );
        assert_eq!(state_39.mileage, 500.0);
        assert_eq!(state_39.bullets, 39.0);

        // Boundary: 39.9 bullets (just below 40.0) must also block hunting
        let mut state_39_9 = sample_state(39.9, 500.0, 2.0);
        go_hunting(&mut state_39_9);
        println!(
            "Boundary 39.9 bullets -> mileage: {}, bullets: {}",
            state_39_9.mileage, state_39_9.bullets
        );
        assert_eq!(state_39_9.mileage, 500.0);
        assert_eq!(state_39_9.bullets, 39.9);

        // Boundary: high skill level clamps score at 0.0 instead of going negative
        let clamped = evaluate_shooting_response("BLAM", "BLAM", 1.2, 3.0);
        println!("Boundary skill clamp (skill 3.0, 1.2s) -> score: {}", clamped);
        assert_eq!(clamped, 0.0);
    }

    #[test]
    fn test_shooting_invalid_inputs() {
        let typo = evaluate_shooting_response("BANG", "BNG", 0.5, 3.0);
        println!("Invalid typo ('BNG' vs 'BANG') -> score: {}", typo);
        assert_eq!(typo, 9.0);

        let wrong_case = evaluate_shooting_response("BANG", "bang", 0.5, 3.0);
        println!("Invalid lowercase ('bang' vs 'BANG') -> score: {}", wrong_case);
        assert_eq!(wrong_case, 9.0);

        let empty = evaluate_shooting_response("WHAM", "", 0.4, 2.0);
        println!("Invalid empty input ('' vs 'WHAM') -> score: {}", empty);
        assert_eq!(empty, 9.0);
    }
}
