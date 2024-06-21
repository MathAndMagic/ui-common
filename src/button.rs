use std::fmt::Display;

use yew::prelude::*;
use yew_nested_router::{components::*, prelude::*};

use crate::Icon;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Color {
    #[default]
    Primary,
    Secondary,
    Dark,
    Danger,
    Success,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Primary => write!(f, "primary"),
            Color::Secondary => write!(f, "secondary"),
            Color::Success => write!(f, "success"),
            Color::Danger => write!(f, "danger"),
            Color::Dark => write!(f, "dark"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Variant {
    #[default]
    Solid,
    Outline,
    Transparent,
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variant::Solid => write!(f, "solid"),
            Variant::Outline => write!(f, "outline"),
            Variant::Transparent => write!(f, "transparent"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Rounded {
    #[default]
    Auto,
    Full,
}

impl Display for Rounded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rounded::Auto => write!(f, "rounded-auto"),
            Rounded::Full => write!(f, "rounded-full"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props<T>
where
    T: Target,
{
    #[prop_or_default]
    pub color: Color,
    #[prop_or_default]
    pub variant: Variant,
    #[prop_or_default]
    pub rounded: Rounded,
    #[prop_or_default]
    pub to: Option<T>,
    #[prop_or(AttrValue::from("button"))]
    pub type_: AttrValue,
    #[prop_or_default]
    pub size: Size,
    #[prop_or_default]
    pub left_icon: Option<Icon>,
    #[prop_or_default]
    pub left_icon_width: Option<u32>,
    #[prop_or_default]
    pub left_icon_height: Option<u32>,
    #[prop_or_default]
    pub right_icon: Option<Icon>,
    #[prop_or_default]
    pub right_icon_width: Option<u32>,
    #[prop_or_default]
    pub right_icon_height: Option<u32>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub text: AttrValue,
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(Button)]
pub fn button<T>(props: &Props<T>) -> Html
where
    T: Target + 'static,
{
    let padding_classes = match props.size {
        Size::Small => "px-2 py-1",
        Size::Medium => "px-2.5 py-1.5",
        Size::Large => "px-3 py-2",
        Size::ExtraLarge => "px-2 py-1",
    };

    let text_size_class = match props.size {
        Size::Small => "text-sm font-medium",
        Size::Medium => "text-sm font-medium",
        Size::Large => "text-sm font-medium",
        Size::ExtraLarge => "text-xs",
    };

    let color_classes = match props.color {
        Color::Primary => "text-white bg-primary-700 hover:bg-primary-600",
        Color::Secondary => "bg-transparent border border-gray-high-950 dark:border-gray-low-300 hover:border-gray-high-700 dark:hover:border-gray-low-50 text-gray-low-500 dark:text-gray-high-950 hover:text-gray-low-50 dark:hover:text-gray-high-700",
        Color::Success => "text-white bg-success-500 hover:bg-success-600",
        Color::Danger => "text-white bg-danger-500 hover:bg-danger-600",
        Color::Dark => "text-gray-800 bg-gray-100 hover:bg-gray-200",
    };

    let class = classes!(
        color_classes,
        "duration-125",
        "ease-in-out",
        "gap-1.5",
        "inline-flex",
        padding_classes,
        text_size_class,
        "rounded-md",
        "transition-colors",
        "items-center",
        props.class.clone(),
    );

    if let Some(to) = &props.to {
        return html! {
            <Link<T>
                class={classes!(class, "inline-block")}
                to={ to.clone() }
            >
                // if let Some(icon) = &props.left_icon {
                //     { icon.clone() }
                // }

                { props.text.clone() }
                { for props.children.iter() }

                // if let Some(icon) = &props.right_icon {
                //     { icon.clone() }
                // }
            </Link<T>>
        };
    }

    return html! {
        <button
            { class }
            type={ props.type_.clone() }
            onclick={props.onclick.clone()}
        >
            // if let Some(icon) = &props.left_icon {
            //     { icon.clone() }
            // }

            { props.text.clone() }
            { for props.children.iter() }

            // if let Some(icon) = &props.right_icon {
            //     { icon.clone() }
            // }
        </button>
    };
}
