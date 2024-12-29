use dioxus::prelude::*;

use crate::structs::Food;

#[derive(Props, PartialEq, Clone)]
pub struct ItemDescriptionProps {
    pub chosen_food: Food,
}

#[component]
pub fn ItemDescription(props: ItemDescriptionProps) -> Element {
    let chosen_food = props.chosen_food;
    rsx! {
        div { class: "bg-slate-100 p-4 my-2 rounded-lg text-xl text-center font-mono",
            div { span { "In a serving of {chosen_food.serving_size}"}, sub {  "{chosen_food.serving_size_unit} " }, span { "there are:"} } ,
            div { "{chosen_food.calories} calories and {chosen_food.protein} protien" },
        }
    }
}
