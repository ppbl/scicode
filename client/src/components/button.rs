use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(
    ButtonProps {
        children,
        class,
        onclick,
    }: &ButtonProps,
) -> Html {
    html!(
        <button
            class={
                format!(
                    "flex items-center justify-center px-4 py-1 rounded
                    text-white bg-indigo-600 hover:bg-indigo-700 text-sm {}",
                    class
                )
            }
            onclick={onclick}
        >
        { children.clone() }
        </button>
    )
}
