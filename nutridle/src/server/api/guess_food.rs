use crate::server::db::get_random_foods;
use crate::structs::Food;
use dioxus::prelude::*;

const FOOD_PICKS_AMOUNT: usize = 4;

#[server(GetRandomFoods)]
pub async fn get_random_food_server() -> Result<Vec<Food>, ServerFnError> {
    let random_foods = get_random_foods(FOOD_PICKS_AMOUNT);
    Ok(random_foods)
}
