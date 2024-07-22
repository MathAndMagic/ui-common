use yew::prelude::*;
use yew_nested_router::{components::*, prelude::*};

use crate::Icon;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Color {
    #[default]
    Primary,
    Secondary,
    Blind,
    Danger,
    Success,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Variant {
    #[default]
    Solid,
    Outline,
    Transparent,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Size {
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Round {
    #[default]
    Auto,
    Full,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Width {
    #[default]
    Auto,
    Full,
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
    pub round: Round,

    #[prop_or_default]
    pub to: Option<T>,

    #[prop_or(AttrValue::from("button"))]
    pub _type: AttrValue,
    #[prop_or_default]
    pub size: Size,
    #[prop_or_default]
    pub width: Width,
    #[prop_or_default]
    pub left_icon: Option<Icon>,
    #[prop_or_default]
    pub right_icon: Option<Icon>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub text: Option<String>,
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,

    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Button<T>(props: &Props<T>) -> Html
where
    T: Target + 'static,
{
    let has_content = props.text.is_some() || !props.children.is_empty();

    let padding_classes = match (props.left_icon, has_content, props.right_icon) {
        (Some(_), false, None) => match props.size {
            // Only icon
            Size::ExtraSmall => "mm-p-1",
            Size::Small => "mm-p-1.5",
            Size::Medium => "mm-p-1.5",
            Size::Large => "mm-p-2",
            Size::ExtraLarge => "mm-p-3",
        },
        (Some(_), true, None) => match props.size {
            // Icon and text
            Size::ExtraSmall => "mm-pl-1.5 mm-pr-2.5 mm-py-1",
            Size::Small => "mm-pl-2 mm-pr-3 mm-py-1.5",
            Size::Medium => "mm-pl-2 mm-pr-3 mm-py-1.5",
            Size::Large => "mm-pl-2.5 mm-pr-3.5 mm-py-2",
            Size::ExtraLarge => "mm-pl-3.5 mm-pr-5 mm-py-3",
        },
        (None, true, Some(_)) => match props.size {
            // Text and icon
            Size::ExtraSmall => "mm-pr-1.5 mm-pl-2.5 mm-py-1",
            Size::Small => "mm-pr-2 mm-pl-3 mm-py-1.5",
            Size::Medium => "mm-pr-2 mm-pl-3 mm-py-1.5",
            Size::Large => "mm-pr-2.5 mm-pl-3.5 mm-py-2",
            Size::ExtraLarge => "mm-pr-3.5 mm-pl-5 mm-py-3",
        },
        (None, true, None) => match props.size {
            // Only text
            Size::ExtraSmall => "mm-px-2.5 mm-py-1",
            Size::Small => "mm-px-3 mm-py-1.5",
            Size::Medium => "mm-px-3 mm-py-1.5",
            Size::Large => "mm-px-3.5 mm-py-2",
            Size::ExtraLarge => "mm-px-5 mm-py-3",
        },
        _ => unreachable!("unexpected combination of left_icon, text, and right_icon"),
    };

    let text_size_class = match props.size {
        Size::ExtraSmall => "mm-text-xs",
        Size::Small => "mm-text-sm",
        Size::Medium => "mm-text-sm",
        Size::Large => "mm-text-sm",
        Size::ExtraLarge => "mm-text-md",
    };

    // TODO: match props.variant for each color
    let color_classes = match props.color {
        Color::Primary => match props.variant {
            Variant::Solid => "mm-text-white mm-bg-primary-700 hover:mm-bg-primary-600 disabled:mm-opacity-30 disabled:hover:mm-bg-primary-700",
            Variant::Transparent => "mm-text-primary-700 mm-bg-transparent mm-border mm-border-primary-700 hover:mm-bg-primary-700 mm-text-white mm-box-border",
            Variant::Outline => "mm-bg-transparent hover:mm-bg-transparent-black-300 dark:hover:mm-bg-transparent-white-200 disabled:mm-opacity-30 mm-text-primary-700 dark:mm-text-primary-500 mm-border mm-border-primary-700 dark:mm-border-primary-500 mm-box-border",
        }
        Color::Secondary => match props.variant {
            Variant::Solid => "mm-bg-transparent-black-300 dark:mm-bg-transparent-white-300 hover:mm-bg-transparent-black-400 dark:hover:mm-bg-transparent-white-400 disabled:mm-opacity-30 disabled:mm-bg-transparent-black-300 disabled:dark:mm-bg-transparent-white-300 mm-text-gray-low-800 dark:mm-text-gray-high-200",
            Variant::Transparent => "mm-bg-transparent hover:mm-bg-transparent-black-300 dark:hover:mm-bg-transparent-white-200 disabled:mm-opacity-30 mm-text-gray-low-100 dark:mm-text-gray-low-200 hover:mm-text-gray-low-400 dark:hover:mm-text-gray-high-700",
            Variant::Outline => "mm-bg-transparent mm-border mm-border-gray-high-950 dark:mm-border-gray-low-300 hover:mm-border-gray-high-700 dark:hover:mm-border-gray-low-50 mm-text-gray-low-500 dark:mm-text-gray-high-950 hover:mm-text-gray-low-50 dark:hover:mm-text-gray-high-700 mm-box-border",
        }
        Color::Blind => match props.variant {
            Variant::Solid => "mm-bg-transparent-black-300 dark:mm-bg-transparent-white-300 hover:mm-bg-transparent-black-400 dark:hover:mm-bg-transparent-white-400 disabled:mm-opacity-30 mm-text-gray-low-100 dark:mm-text-gray-low-200",
            Variant::Outline => "mm-bg-transparent hover:mm-bg-transparent-black-300 dark:hover:mm-bg-transparent-white-200 disabled:mm-opacity-30 mm-text-gray-low-100 dark:mm-text-gray-low-200 mm-border mm-border-gray-high-800 dark:mm-border-gray-low-300 mm-box-border",
            Variant::Transparent => "mm-bg-transparent hover:mm-bg-transparent-black-300 dark:hover:mm-bg-transparent-white-200 disabled:mm-opacity-30 mm-text-gray-low-100 dark:mm-text-gray-low-200",
        },
        Color::Success => "mm-text-white mm-bg-success-500 hover:mm-bg-success-600",
        Color::Danger => "mm-text-white mm-bg-danger-500 hover:mm-bg-danger-600",
    };

    let rounded_class = match props.round {
        Round::Auto => match props.size {
            Size::ExtraSmall => "mm-rounded",
            Size::Small => "mm-rounded",
            Size::Medium => "mm-rounded-md",
            Size::Large => "mm-rounded-lg",
            Size::ExtraLarge => "mm-rounded-lg",
        },
        Round::Full => "mm-rounded-full",
    };

    let width_class = match props.width {
        Width::Auto => classes!("mm-w-auto"),
        Width::Full => classes!("mm-w-full", "mm-justify-center"),
    };

    let class = classes!(
        color_classes,
        "mm-duration-125",
        "mm-ease-in-out",
        "mm-gap-1.5",
        "mm-inline-flex",
        padding_classes,
        text_size_class,
        rounded_class,
        width_class,
        "mm-transition-colors",
        "mm-items-center",
        "mm-outline-none",
        props.class.clone(),
    );

    let icon_size_class = match props.size {
        Size::ExtraSmall => "mm-text-base",
        Size::Small => "mm-text-base",
        Size::Medium => "text-xl",
        Size::Large => "text-xl",
        Size::ExtraLarge => "text-2xl",
    };

    let icon_class = classes!(
        icon_size_class,
        "mm-inline-block",
        "mm-justify-center",
        "mm-items-center",
        "mm-flex",
    );

    if let Some(to) = &props.to {
        return html! {
            <Link<T>
                class={classes!(class, "mm-inline-block")}
                to={ to.clone() }
            >
                if let Some(icon) = &props.left_icon {
                    <span class={icon_class}>{ *icon }</span>
                }

                { props.text.clone() }
                { for props.children.iter() }

                if let Some(icon) = &props.right_icon {
                    { *icon }
                }
            </Link<T>>
        };
    }

    html! {
        <button
            { class }
            type={ props._type.clone() }
            onclick={props.onclick.clone()}
            disabled={props.disabled}
            name={props.name.clone()}
            value={props.value.clone()}
        >
            if let Some(icon) = &props.left_icon {
                <span class={icon_class}>{ *icon }</span>
            }

            { props.text.clone() }
            { for props.children.iter() }

            if let Some(icon) = &props.right_icon {
                { *icon }
            }
        </button>
    }
}
