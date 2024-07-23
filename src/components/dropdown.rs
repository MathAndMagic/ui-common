use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::{use_click_away, use_event_with_window, use_window_size};

#[derive(Debug, Clone, PartialEq, Default)]
struct Coordinates {
    top: f64,
    left: f64,
}

#[derive(PartialEq, Properties)]
pub struct DropdownItemTextProps {
    pub children: Children,
}

#[function_component]
pub fn DropdownItemText(DropdownItemTextProps { children }: &DropdownItemTextProps) -> Html {
    html! {
        <div class="mm-flex mm-grow">
            { for children.iter() }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct DropdownItemIconProps {
    pub children: Children,
}

#[function_component]
pub fn DropdownItemIcon(DropdownItemIconProps { children }: &DropdownItemIconProps) -> Html {
    html! {
        <div class="mm-flex mm-min-w-4 mm-min-h-4">
            { for children.iter() }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct DropdownItemProps {
    pub children: Children,
}

#[function_component]
pub fn DropdownItem(DropdownItemProps { children }: &DropdownItemProps) -> Html {
    html! {
        <div class="mm-flex mm-flex-row mm-items-center mm-py-2 mm-px-3 mm-text-gray-low-400 dark:mm-text-gray-high-700 hover:mm-bg-gray-high-50 hover:dark:mm-bg-gray-low-600 hover:mm-text-gray-low-800 hover:dark:mm-text-gray-high-200 mm-cursor-pointer mm-gap-3">
            { for children.iter() }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct DropdownProps {
    pub anchor_ref: Option<HtmlElement>,
    pub open: bool,

    #[prop_or(8)]
    pub offset: i16,

    pub on_close: Callback<()>,
    pub children: Children,
}

#[function_component]
pub fn Dropdown(
    DropdownProps {
        open,
        offset,
        anchor_ref,
        on_close,
        children,
    }: &DropdownProps,
) -> Html {
    let node = use_node_ref();
    let coordinates = use_state(Coordinates::default);
    let (window_size_x, _window_size_y) = use_window_size();

    use_effect_with((anchor_ref.clone(), coordinates.clone(), window_size_x), {
        |(anchor, coordinates, _window_x_size)| {
            if let Some(anchor) = anchor {
                let rect = anchor.get_bounding_client_rect();

                coordinates.set(Coordinates {
                    top: rect.bottom(),
                    left: rect.left(),
                });
            }

            move || {}
        }
    });

    use_click_away(node.clone(), {
        let on_close = on_close.clone();

        move |_event: Event| {
            on_close.emit(());
        }
    });

    use_event_with_window("keydown", {
        let on_close = on_close.clone();

        move |event: KeyboardEvent| {
            if event.key() == "Escape" {
                on_close.emit(());
            }
        }
    });

    if !open || anchor_ref.is_none() {
        return html! {};
    }

    html! {
        <div
            ref={node}
            style={format!("top: {top}px; left: {left}px; margin-top: {offset}px;", top = coordinates.top, left = coordinates.left, offset = offset)}
            class="mm-absolute mm-flex mm-flex-col mm-bg-gray-high-100 dark:mm-bg-gray-low-700 mm-p-2 mm-rounded-md mm-shadow-md mm-border mm-border-transparent-black-400 dark:mm-border-transparent-white-400 mm-w-96"
        >
            { for children.iter() }
        </div>
    }
}
