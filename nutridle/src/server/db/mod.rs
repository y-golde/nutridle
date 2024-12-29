use crate::structs::Food;
use dioxus::html::u;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use std::fs;
use std::sync::Mutex;

const NUTRI_OUTPUT: &str = "db/nutri_output.json";

fn get_all_foods() -> Result<Vec<Food>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(NUTRI_OUTPUT)?;
    let foods: Vec<Food> = serde_json::from_str(&data)?;
    Ok(foods)
}

lazy_static! {
    pub static ref ALL_FOODS_STATE: Mutex<Vec<Food>> = Mutex::new(vec![]);
}

pub fn load_all_foods() {
    let all_foods = get_all_foods().unwrap();
    let mut all_foods_state = ALL_FOODS_STATE.lock().unwrap();
    *all_foods_state = all_foods;
}

pub fn get_random_foods(count: usize) -> Vec<Food> {
    let all_foods_state = ALL_FOODS_STATE.lock().unwrap();
    if all_foods_state.len() == 0 {
        load_all_foods();
    }

    let mut rng = rand::thread_rng();
    let random_foods = all_foods_state
        .choose_multiple(&mut rng, count)
        .cloned()
        .collect::<Vec<Food>>();
    random_foods
}
