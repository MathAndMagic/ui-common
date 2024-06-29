use yew::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub enum Level {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(PartialEq, Properties)]
pub struct HeadingProps {
    #[prop_or_default]
    pub level: Level,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Heading(props: &HeadingProps) -> Html {
    let HeadingProps { level, children } = props;

    let tag = match level {
        Level::H1 => "h1",
        Level::H2 => "h2",
        Level::H3 => "h3",
        Level::H4 => "h4",
        Level::H5 => "h5",
        Level::H6 => "h6",
    };

    let class = match level {
        Level::H1 => "mm-text-4xl mm-font-bold",
        Level::H2 => "mm-text-3xl mm-font-bold",
        Level::H3 => "mm-text-2xl mm-font-bold",
        Level::H4 => "mm-text-xl mm-font-bold",
        Level::H5 => "mm-text-lg mm-font-bold",
        Level::H6 => "mm-text-md mm-font-bold",
    };

    html! {
        <@{tag} {class}>
            { for children.iter() }
        </@>
    }
}
