
//! Part 3: Travel Decisions.
//!
//! Every function below is a skeleton: it compiles and returns a harmless default so the
//! other parts can build against it. Replace the body with your own code. Keep the
//! signature unless your team and instructor agree to change it.

use crate::types::{EatingLevel, GameOver, GameState};

/// What the riders *look* like. The truth can differ (see `resolve_rider_encounter`).
/// This type belongs to Part 3 alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hostility {
    Hostile,
    Friendly,
}

/// Asks how well to eat, re-asking until the choice is valid and affordable. Returns the
/// level settled on. Returns `Err(GameOver::Starved)` when food is below 13.
///
/// Skeleton returns `Ok(EatingLevel::Moderate)` without prompting.
pub fn choose_eating_level(state: &mut GameState) -> Result<EatingLevel, GameOver> {
  
	if state.food < 13.0 {
           return Err(GameOver::Starved); 
}
           loop {
 let n = crate::io::ask_number("DO YOU WANT TO EAT (1) POORLY (2) MODERATELY\nOR (3) WELL");
	let level = if n == 1.0 {
           EatingLevel::Poor
       } else if n == 2.0 {
           EatingLevel::Moderate 
       } else if n == 3.0 { 
           EatingLevel::Well 
       } else {
           continue;
       };
        if consume_food(state,level) .is_ok() {
            return Ok(level);
       }
          println!("YOU CAN'T EAT THAT WELL");
       }
}
/// Deducts the food cost of an eating level if it can be afforded. `Err(())` means it
/// could not be afforded and nothing changed.
///
/// Skeleton returns `Ok(())` without changing anything.
pub fn consume_food(state: &mut GameState, level: EatingLevel) -> Result<(), ()> {
	let cost= match level{
    EatingLevel::Poor => 13.0,
    EatingLevel::Moderate  => 18.0,
    EatingLevel::Well => 23.0,
};
        if state.food < cost { return Err(());
 }
         state.food -= cost;
            Ok(())
}


/// Adds this turn's travel distance to `mileage`.
///
/// Skeleton does nothing.
pub fn calculate_daily_mileage(state: &mut GameState) {
       state.mileage += mileage_gain(state.oxen_spend, crate::rng::rnd())
}
    fn mileage_gain(oxen_spend: f64, roll: f64) ->f64 {
         200.0 + (oxen_spend-220.0)/3.0 +10.0 * roll
 }
/// Rolls whether riders appear this turn. `None` means no encounter; otherwise returns
/// how the riders look.
///
/// Skeleton returns `None` (never an encounter).
pub fn rider_encounter_roll(mileage: f64) -> Option<Hostility> {
    None
}

/// Plays out the rider encounter: tactic menu, outcome, losses. Returns
/// `Err(GameOver::RiderMassacre)` if the bullets run out against hostile riders.
///
/// Skeleton does nothing and returns `Ok(())`.
pub fn resolve_rider_encounter(state: &mut GameState, looks: Hostility) -> Result<(), GameOver> {
    Ok(())
}
#[cfg(test)]
mod tests{
use super:: *;
#[test]
fn cannot_afford_well() {
	let mut state = GameState { food: 20.0,..Default::default()};
     assert_eq!( consume_food(&mut state, EatingLevel::Well) , Err(()));
     assert_eq!(state.food,20.0);
}
#[test]
fn can_afford_moderate() {
        let mut state = GameState { food: 30.0,..Default::default()};
     assert_eq!(consume_food(&mut state, EatingLevel::Moderate),Ok(()));
     assert_eq!(state.food,12.0)
     }
#[test]
fn mileage_gain_test() {
     assert_eq! (mileage_gain(280.0,0.5),225.0);
}
#[test] 
fn mileage_gain_negative_output() {
     assert_eq! (mileage_gain(190.0, 0.0), 190.0);
}
#[test]
fn daily_mileage_test() {
	let mut state = GameState { mileage: 100.0, oxen_spend: 280.0, ..Default::default() };
      
calculate_daily_mileage(&mut state);
     assert! (state.mileage >= 320.0 && state.mileage < 330.0);
     }
 
#[test]
fn starved_check () {
        let mut state = GameState { food: 12.0,..Default::default()};
   assert_eq!(choose_eating_level(&mut state), Err(GameOver::Starved));
    }
}

