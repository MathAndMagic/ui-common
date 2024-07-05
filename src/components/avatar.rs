use yew::prelude::*;
use yew_nested_router::{components::*, prelude::*};

use crate::Icon;

#[derive(PartialEq, Default, Clone)]
pub enum Size {
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

#[derive(PartialEq, Default, Clone)]
pub enum Variant {
    #[default]
    Icon,
    Letter,
    Image,
}

/// Possible connection status.
#[derive(Clone, Debug, PartialEq)]
pub enum ConnectionStatus {
    Connecting,
    Open,
    Closed,
}

#[derive(PartialEq, Properties)]
pub struct Props<T>
where
    T: Target,
{
    /// Variant of the avatar.
    #[prop_or_default]
    pub variant: Variant,

    /// Size of the avatar.
    #[prop_or_default]
    pub size: Size,

    /// Icon to display.
    #[prop_or(Icon::USER)]
    pub icon: Icon,

    /// Text to display.
    #[prop_or_default]
    pub name: AttrValue,

    /// Target to navigate to.
    #[prop_or_default]
    pub to: Option<T>,

    /// Click event handler.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    /// Websocket connection status.
    #[prop_or_default]
    pub connection_status: Option<ConnectionStatus>,

    /// List of classes to apply.
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn Avatar<T>(props: &Props<T>) -> Html
where
    T: Target + 'static,
{
    let dimension_classes = match props.size {
        Size::ExtraSmall => "mm-w-6 mm-h-6",
        Size::Small => "mm-w-7 mm-h-7",
        Size::Medium => "mm-w-8 mm-h-8",
        Size::Large => "mm-w-9 mm-h-9",
        Size::ExtraLarge => "mm-w-12 mm-h-12",
    };

    let color_classes = match props.variant {
        Variant::Icon | Variant::Letter => "mm-border mm-border-transparent-black-400 dark:mm-border-transparent-white-400 mm-text-gray-low-100 dark:mm-text-gray-low-200",
        Variant::Image => "mm-border mm-border-transparent",
    };

    let bg_classes = "mm-bg-gray-high-300 dark:mm-bg-gray-low-900";

    let class = classes!(
        "mm-inline-flex",
        bg_classes,
        dimension_classes,
        color_classes,
        "mm-rounded-full",
        "mm-transition-colors",
        "mm-duration-125",
        "mm-ease-in-out",
        "mm-items-center",
        "mm-justify-center",
        "mm-relative",
        props.class.clone(),
    );

    // TODO: sizing only implemented for the default (`Medium`) variant. Implement for others.
    let connection_status = match props.connection_status {
        Some(ConnectionStatus::Connecting) => html! {
            <span class="mm-absolute mm-w-[7px] mm-h-[7px] mm-bg-amber-600 dark:mm-bg-amber-500 mm-rounded-full mm-bottom-px mm-right-px mm-border-[0.5px] mm-border-black-800 dark:mm-border-white-800 mm-animate-ping"></span>
        },
        Some(ConnectionStatus::Open) => html! {
            <span class="mm-absolute mm-w-[7px] mm-h-[7px] mm-bg-green-700 mm-rounded-full mm-bottom-px mm-right-px mm-border-[0.5px] mm-border-black-800 dark:mm-border-white-800"></span>
        },
        Some(ConnectionStatus::Closed) => html! {
            <span class="mm-absolute mm-w-[7px] mm-h-[7px] mm-bg-gray-high-900 dark:mm-bg-gray-low-400 mm-rounded-full mm-bottom-px mm-right-px mm-border-[0.5px] mm-border-black-800 dark:mm-border-white-800"></span>
        },
        None => html! {},
    };

    if let Some(to) = &props.to {
        return html! {
            <Link<T>
                class={classes!(class, "mm-inline-block")}
                to={ to.clone() }
            >
                <AvatarContent ..props.into() />
                { connection_status }
            </Link<T>>
        };
    }

    return html! {
        <button
            { class }
            type="button"
            onclick={props.onclick.clone()}
        >
            <AvatarContent ..props.into() />
            { connection_status }
        </button>
    };
}

#[derive(PartialEq, Properties)]
pub struct AvatarContentProps {
    /// Variant of the avatar.
    #[prop_or_default]
    pub variant: Variant,

    /// Icon to display.
    #[prop_or(Icon::USER)]
    pub icon: Icon,

    /// Text to display.
    #[prop_or_default]
    pub name: AttrValue,
}

impl<T> From<&Props<T>> for AvatarContentProps
where
    T: Target,
{
    fn from(props: &Props<T>) -> Self {
        AvatarContentProps {
            variant: props.variant.clone(),
            icon: props.icon.clone(),
            name: props.name.clone(),
        }
    }
}

#[function_component]
fn AvatarContent(props: &AvatarContentProps) -> Html {
    html! {
        if props.variant == Variant::Icon {
            <span class="mm-w-5 mm-min-h-5 mm-inline-block mm-justify-center mm-items-center mm-flex mm-text-xl">{ props.icon.clone() }</span>
        } else if props.variant == Variant::Letter {
            <span class="mm-text-sm mm-uppercase">{ props.name.clone().chars().next().unwrap_or_default().to_string() }</span>
        } else if props.variant == Variant::Image {
            <img
                class="mm-w-10 mm-h-10 mm-rounded-full"
                src="#"
                alt="Avatar"
            />
        }
    }
}
