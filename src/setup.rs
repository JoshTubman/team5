//! Part 1: Foundation & Setup.
//!
//! Every function below is a skeleton: it compiles and returns a harmless default so the
//! other parts can build against it. Replace the body with your own code. Keep the
//! signature unless your team and instructor agree to change it.
use crate::io;
use crate::types::{GameOver, GameState};

/// Prints the skill menu, reads the choice with `io::ask_number`, and returns it.
/// A value above 5 silently becomes 0 (see the Part 1 page).
///
/// Skeleton returns 3.0 ("fair to middlin'").
pub fn get_shooting_skill_level() -> f64 {
    3.0
}

/// Runs the whole initial-purchase sequence and returns the starting `GameState`.
///
/// Skeleton returns a generic, plausible starting state so other parts can be tested.

pub fn purchase_supplies(skill_level: f64) -> GameState {
    loop {
        // Purchase oxen
        let oxen_spend = loop {
            let amount = io::ask_number(
                "HOW MUCH DO YOU WANT TO SPEND ON YOUR OXEN TEAM"
            );

            if amount < 200.0 {
                println!("NOT ENOUGH");
            } else if amount > 300.0 {
                println!("TOO MUCH");
            } else {
                break amount;
            }
        };

        // Purchase food
        let food = loop {
            let amount = io::ask_number(
                "HOW MUCH DO YOU WANT TO SPEND ON FOOD"
            );

            if amount < 0.0 {
                println!("IMPOSSIBLE");
            } else {
                break amount;
            }
        };

        // Purchase ammunition
        let ammunition = loop {
            let amount = io::ask_number(
                "HOW MUCH DO YOU WANT TO SPEND ON AMMUNITION"
            );

            if amount < 0.0 {
                println!("IMPOSSIBLE");
            } else {
                break amount;
            }
        };

        // Purchase clothing
        let clothing = loop {
            let amount = io::ask_number(
                "HOW MUCH DO YOU WANT TO SPEND ON CLOTHING"
            );

            if amount < 0.0 {
                println!("IMPOSSIBLE");
            } else {
                break amount;
            }
        };

        // Purchase miscellaneous supplies
        let misc_supplies = loop {
            let amount = io::ask_number(
                "HOW MUCH DO YOU WANT TO SPEND ON MISCELLANEOUS SUPPLIES"
            );

            if amount < 0.0 {
                println!("IMPOSSIBLE");
            } else {
                break amount;
            }
        };

        // Calculate total spending
        let total = oxen_spend + food + ammunition
            + clothing + misc_supplies;

        // Restart if the player overspends
        if total > 700.0 {
            println!(
                "YOU OVERSPENT--YOU ONLY HAD $700 TO SPEND. BUY AGAIN"
            );
            continue;
        }

        // remaining cash
        let cash = 700.0 - total;

        // Convert ammo dollars into bullets
        let bullets = ammunition * 50.0;

        println!(
            "AFTER ALL YOUR PURCHASES, YOU NOW HAVE {} DOLLARS LEFT",
            io::fmt_num(cash)
        );

        println!("MONDAY MARCH 29 1847");

        return GameState {
            food,
            bullets,
            clothing,
            misc_supplies,
            cash,
            oxen_spend,
            skill_level,
            ..GameState::default()
        };
    }
}


/// Advances to the next turn and prints its date. Returns `Err(GameOver::TrailTooLong)`
/// when the trip runs past the last date in the table.
///
/// Skeleton does nothing and returns `Ok(())`.

pub fn advance_turn_date(state: &mut GameState) -> Result<(), GameOver> {
    let dates = [
        "APRIL 12",
        "APRIL 26",
        "MAY 10",
        "MAY 24",
        "JUNE 7",
        "JUNE 21",
        "JULY 5",
        "JULY 19",
        "AUGUST 2",
        "AUGUST 16",
        "AUGUST 30",
        "SEPTEMBER 13",
        "SEPTEMBER 27",
        "OCTOBER 11",
        "OCTOBER 25",
        "NOVEMBER 8",
        "NOVEMBER 22",
        "DECEMBER 6",
        "DECEMBER 20",
    ];

    state.turn_number += 1;

    if state.turn_number >= 20  {
        println!("YOU HAVE BEEN ON THE TRAIL TOO LONG ------");
        println!("YOUR FAMILY DIES IN THE FIRST BLIZZARD OF WINTER");

        return Err(GameOver::TrailTooLong);
    }

    let date = dates[(state.turn_number - 1) as usize];

    println!("\nMONDAY {} 1847\n", date);

    Ok(())
}


/// Start-of-turn cleanup and the status table: clamp negatives, low-food warning, floor
/// values, record `turn_start_mileage`, print mileage and the resource table.
///
/// Skeleton does nothing.
pub fn print_status_table(state: &mut GameState) {}

/// Charges the $20 doctor's bill when an illness or injury flag is set. Returns
/// `Err(GameOver::CantAffordDoctor)` when cash cannot cover it.
///
/// Skeleton does nothing and returns `Ok(())`.
pub fn apply_doctor_bill(state: &mut GameState) -> Result<(), GameOver> {
    Ok(())
}
