use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MarkdownProps {
    pub source: String,
    #[prop_or_default]
    pub class: String,
}

#[function_component(Markdown)]
pub fn markd(MarkdownProps { class, source }: &MarkdownProps) -> Html {
    let div = gloo::utils::document().create_element("div").unwrap();
    div.set_class_name(class);
    div.set_inner_html(markdown::to_html(source).as_str());
    Html::VRef(div.into())
}
