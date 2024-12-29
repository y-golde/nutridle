use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BrandedFoodContainer {
    pub BrandedFoods: Vec<FoodItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FoodItem {
    pub foodClass: String,
    pub description: String,
    pub foodNutrients: Vec<FoodNutrient>,
    pub foodAttributes: Vec<FoodAttribute>,
    pub modifiedDate: String,
    pub availableDate: String,
    pub marketCountry: String,
    pub brandOwner: String,
    pub gtinUpc: String,
    pub dataSource: String,
    pub ingredients: String,
    pub servingSize: f32,
    pub servingSizeUnit: String,
    pub householdServingFullText: String,
    pub labelNutrients: LabelNutrients,
    pub tradeChannels: Vec<String>,
    // pub microbes: Vec<String>,
    pub brandedFoodCategory: String,
    pub dataType: String,
    pub fdcId: u64,
    pub publicationDate: String,
    pub foodUpdateLog: Vec<FoodUpdateLog>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FoodNutrient {
    #[serde(rename = "type")]
    pub nutrient_type: String,
    pub id: u64,
    pub nutrient: Nutrient,
    pub foodNutrientDerivation: FoodNutrientDerivation,
    pub amount: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Nutrient {
    pub id: u64,
    pub number: String,
    pub name: String,
    pub rank: Option<u32>,
    pub unitName: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FoodNutrientDerivation {
    pub code: Option<String>,
    pub description: Option<String>,
    pub foodNutrientSource: FoodNutrientSource,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FoodNutrientSource {
    pub id: Option<u64>,
    pub code: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FoodAttribute {
    pub id: u64,
    pub name: Option<String>,
    pub value: String,
    pub foodAttributeType: FoodAttributeType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FoodAttributeType {
    pub id: u64,
    pub name: Option<String>,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LabelNutrients {
    pub fat: Option<NutrientValue>,
    pub saturatedFat: Option<NutrientValue>,
    pub transFat: Option<NutrientValue>,
    pub cholesterol: Option<NutrientValue>,
    pub sodium: Option<NutrientValue>,
    pub carbohydrates: Option<NutrientValue>,
    pub fiber: Option<NutrientValue>,
    pub sugars: Option<NutrientValue>,
    pub protein: Option<NutrientValue>,
    pub calcium: Option<NutrientValue>,
    pub iron: Option<NutrientValue>,
    pub calories: Option<NutrientValue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NutrientValue {
    pub value: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FoodUpdateLog {
    pub foodClass: String,
    pub description: String,
    pub foodAttributes: Vec<FoodAttributeLog>,
    pub dataType: String,
    pub fdcId: u64,
    pub publicationDate: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FoodAttributeLog {
    pub id: u64,
    pub name: Option<String>,
    pub value: String,
    pub foodAttributeType: FoodAttributeTypeLog,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FoodAttributeTypeLog {
    pub id: u64,
}
