mod FoodData;

use crate::structs::Food;
use serde_json::to_writer;
use std::fs;
use FoodData::{BrandedFoodContainer, NutrientValue};

const NUTRI_INPUT: &str = "db/branded_input.json";
const NUTRI_OUTPUT: &str = "db/nutri_output.json";

fn format_label_nutrient(nutrient: &Option<NutrientValue>) -> f32 {
    match nutrient {
        Some(n) => n.value,
        None => -1.0,
    }
}

pub fn seed_db() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(NUTRI_INPUT)?;
    let foods_data: BrandedFoodContainer = serde_json::from_str(&data)?;

    let mut foods_vec: Vec<Food> = vec![];

    for food in &foods_data.BrandedFoods {
        let label_nutrients = &food.labelNutrients;
        let f = Food {
            id: food.fdcId,
            description: food.description.clone(),
            serving_size: food.servingSize,
            serving_size_unit: food.servingSizeUnit.clone(),
            brand_owner: food.brandOwner.clone(),
            ingredients: food.ingredients.clone(),

            fat: format_label_nutrient(&label_nutrients.fat),
            saturatedFat: format_label_nutrient(&label_nutrients.saturatedFat),
            transFat: format_label_nutrient(&label_nutrients.transFat),
            cholesterol: format_label_nutrient(&label_nutrients.cholesterol),
            sodium: format_label_nutrient(&label_nutrients.sodium),
            carbohydrates: format_label_nutrient(&label_nutrients.carbohydrates),
            fiber: format_label_nutrient(&label_nutrients.fiber),
            sugars: format_label_nutrient(&label_nutrients.sugars),
            protein: format_label_nutrient(&label_nutrients.protein),
            calcium: format_label_nutrient(&label_nutrients.calcium),
            iron: format_label_nutrient(&label_nutrients.iron),
            calories: format_label_nutrient(&label_nutrients.calories),
        };
        foods_vec.push(f);
    }

    println!("{:?}", foods_vec[0]);

    let file = fs::File::create(NUTRI_OUTPUT)?;
    to_writer(file, &foods_vec)?;

    Ok(())
}
