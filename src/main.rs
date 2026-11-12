//! The Oregon Trail (MECC, 1978) ported from BASIC to Rust.
//!
//! This is the starter project. Each part lives in its own file so teammates rarely edit
//! the same file. The turn loop is not provided: your team writes it in `main` at
//! Checkpoint 3, when the four parts are integrated.

// The skeletons are not called from anywhere yet, and their parameters are unused.
// Remove these allows once your team has integrated the parts.
//! The Oregon Trail (MECC, 1978) ported from BASIC to Rust.
//!
//! This is the starter project. Each part lives in its own file so teammates rarely edit
//! the same file. The turn loop is not provided: your team writes it in `main` at
//! Checkpoint 3, when the four parts are integrated.

// The skeletons are not called from anywhere yet, and their parameters are unused.
// Remove these allows once your team has integrated the parts.
#![allow(dead_code, unused_variables)]
mod io;
mod setup;
mod fort_hunting;
mod travel;
mod rng;
mod types;


fn main() {
    // Test turn 1
    println!("--- TESTING TURN 1 ---");
    let mut state = types::GameState::default();

    setup::advance_turn_date(&mut state);

    // Test turn 19
    println!("--- TESTING TURN 19 ---");
    state.turn_number = 18;

    setup::advance_turn_date(&mut state);

    // Test turn 20
    println!("--- TESTING TURN 20 ---");
    state.turn_number = 19;

    match setup::advance_turn_date(&mut state) {
        Ok(()) => println!("Unexpected: game continued."),
        Err(types::GameOver::TrailTooLong) => {
            println!("SUCCESS: Game ended because the player reached turn 20.");
        }
        Err(_) => println!("Unexpected game-over condition."),
    }
}
