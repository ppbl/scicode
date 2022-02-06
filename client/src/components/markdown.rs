use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MarkdownProps {
    pub source: String,
}

#[function_component(Markdown)]
pub fn markd(props: &MarkdownProps) -> Html {
    let div = gloo::utils::document().create_element("div").unwrap();
    div.set_inner_html(markdown::to_html(&props.source).as_str());
    Html::VRef(div.into())
}
