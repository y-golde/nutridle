use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Food {
    // food attributres
    pub id: u64,
    pub description: String,
    pub serving_size: f32,
    pub serving_size_unit: String,
    pub brand_owner: String,
    pub ingredients: String,

    // label nutrients
    pub fat: f32,
    pub saturatedFat: f32,
    pub transFat: f32,
    pub cholesterol: f32,
    pub sodium: f32,
    pub carbohydrates: f32,
    pub fiber: f32,
    pub sugars: f32,
    pub protein: f32,
    pub calcium: f32,
    pub iron: f32,
    pub calories: f32,
}
