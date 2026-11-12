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
    println!("--- TESTING PURCHASE_SUPPLIES ---");

    let state = setup::purchase_supplies(3.0);

    println!("\n--- PURCHASE RESULTS ---");
    println!("Oxen spending: {}", state.oxen_spend);
    println!("Food: {}", state.food);
    println!("Bullets: {}", state.bullets);
    println!("Clothing: {}", state.clothing);
    println!("Misc supplies: {}", state.misc_supplies);
    println!("Remaining cash: {}", state.cash);
}
