mod FoodOption;
mod ItemDescription;

use std::collections::HashSet;

use crate::{server::api::get_random_food_server, structs::Food};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use rand::seq::SliceRandom;
use FoodOption::FoodOption;
use ItemDescription::{ItemDescription, ItemDescriptionProps};

const MAX_TRIES: usize = 2;

#[component]
pub fn Game() -> Element {
    let random_foods_future = use_resource(move || get_random_food_server());
    let tries = use_signal::<HashSet<u64>>(|| HashSet::new());
    let mut chosen_food_id = use_signal::<u64>(|| 0);

    use_effect(move || {
        if let Some(Ok(foods)) = &*random_foods_future.read_unchecked() {
            let random_food = foods.choose(&mut rand::thread_rng()).unwrap();
            chosen_food_id.set(random_food.id);
        }
    });

    match &*random_foods_future.read_unchecked() {
        Some(Ok(foods)) => {
            let chosen_food_id_value = *chosen_food_id.read_unchecked();
            if chosen_food_id_value == 0 {
                return rsx! {"Loading items"};
            }
            let chosen_food = foods
                .iter()
                .find(|food| food.id == chosen_food_id_value)
                .unwrap();

            rsx! {
                div {
                    h1 { class: "text-4xl text-center", "Guess the food!" }
                    p { class: "text-slate-500 italic text-center","Choose the food that fits this description:" }
                    ItemDescription { chosen_food: chosen_food.clone() }
                }

                div  {
                    class: "grid grid-col-1 gap-2 mt-2",
                    for food in foods {
                        FoodOption { food: food.clone(), is_selected: food.id == chosen_food.id }
                    }
                }
            }
        }
        Some(Err(err)) => {
            // if there was an error, render the error
            rsx! {"An error occurred while fetching stories {err}"}
        }
        None => {
            // if the future is not resolved yet, render a loading message
            rsx! {"Loading items"}
        }
    }
}

// list of ingredients
// 4 random foods to choose from
