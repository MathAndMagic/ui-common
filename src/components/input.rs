use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Variant {
    #[default]
    Standard,
    Round,
    Underline,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub variant: Variant,
    #[prop_or(AttrValue::from("text"))]
    pub _type: AttrValue,
    #[prop_or_default]
    pub size: Size,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub onkeydown: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub onkeyup: Callback<KeyboardEvent>,
}

#[function_component]
pub fn Input(props: &Props) -> Html {
    let padding_classes = match props.variant {
        Variant::Standard => match props.size {
            Size::Small => "mm-px-2 mm-py-0.5",
            Size::Medium => "mm-px-2 mm-py-1",
            Size::Large => "mm-px-2.5 mm-py-1.5",
            Size::ExtraLarge => "mm-px-3 mm-py-2",
        },
        Variant::Round => match props.size {
            Size::Small => "mm-px-3.5 mm-py-0.5",
            Size::Medium => "mm-px-3.5 mm-py-1",
            Size::Large => "mm-px-4 mm-py-1.5",
            Size::ExtraLarge => "mm-px-5 mm-py-2",
        },
        Variant::Underline => match props.size {
            Size::Small => "mm-py-0.5",
            Size::Medium => "mm-py-1",
            Size::Large => "mm-py-1.5",
            Size::ExtraLarge => "mm-py-3",
        },
    };

    let text_size_classes = match props.size {
        Size::Small => "mm-text-md mm-font-normal",
        Size::Medium => "mm-text-md mm-font-normal",
        Size::Large => "mm-text-md mm-font-normal",
        Size::ExtraLarge => "mm-text-3xl mm-font-medium",
    };

    let border_classes = match props.variant {
        Variant::Standard | Variant::Round => "mm-border mm-border-transparent-black-700 dark:mm-border-transparent-white-600 focus:mm-border-transparent-black-800 dark:focus:mm-border-transparent-white-800",
        Variant::Underline => "mm-border-b mm-border-b-transparent-black-400 dark:mm-border-b-transparent-white-400 focus:mm-border-b-transparent-black-800 dark:focus:mm-border-b-transparent-white-800",
    };

    let border_radius_classes = match props.variant {
        Variant::Standard => match props.size {
            Size::Small | Size::Medium | Size::Large => "mm-rounded-md",
            Size::ExtraLarge => "mm-rounded-lg",
        },
        Variant::Round => "mm-rounded-full",
        Variant::Underline => "mm-rounded-none",
    };

    let class = classes!(
        "mm-w-full",
        "mm-bg-transparent",
        "autofill:mm-bg-transparent",
        "disabled:mm-bg-gray-high-300",
        "dark:disabled:mm-bg-gray-low-900",
        "mm-text-gray-low-800",
        "dark:mm-text-gray-high-200",
        "placeholder:mm-text-gray-high-900",
        "placeholder:dark:mm-text-gray-low-400",
        "mm-transition-colors",
        "mm-duration-125",
        "mm-ease-in-out",
        "mm-outline-none",
        "mm-select-auto",
        border_classes,
        border_radius_classes,
        padding_classes,
        text_size_classes,
        props.class.clone(),
    );

    html! {
        <input
            { class }
            type={ props._type.clone() }
            placeholder={ props.placeholder.clone() }
            name={ props.name.clone() }
            required={ props.required }
            value={ props.value.clone() }
            onkeydown={ props.onkeydown.clone() }
            onkeyup={ props.onkeyup.clone() }
        />
    }
}
