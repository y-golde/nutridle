use dioxus::prelude::*;

use crate::structs::Food;

#[derive(Props, PartialEq, Clone)]
pub struct FoodItemProps {
    pub food: Food,
    pub is_selected: bool,
}

#[component]
pub fn FoodOption(props: FoodItemProps) -> Element {
    let food = props.food;
    let is_selected = props.is_selected;

    let mut selected = use_signal(|| false);

    let button_base_class = "p-3 rounded-lg";

    let button_color = match *selected.read_unchecked() {
        true => match is_selected {
            true => "bg-green-500 hover:bg-green-600",
            false => "bg-red-500 hover:bg-red-600",
        },
        false => "bg-purple-200 hover:bg-purple-300",
    };

    let button_class = format!("{} {}", button_base_class, button_color);

    rsx! {
        div {
            class: "text-center",
            button { class: button_class, onclick: move |_| {
                selected.set(true);
            },
            "{food.description}" }
        }
    }
}
