use web_sys::HtmlElement;
use yew::prelude::*;

// Describes the size of the tooltip arrow (based on the largest side)
const TOOLTIP_SIZE: f64 = 8.0;

#[derive(Debug, Clone, PartialEq, Default)]
struct Coordinates {
    top: f64,
    left: f64,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TooltipPosition {
    Top,

    #[default]
    Right,
    Bottom,
    Left,
}

#[derive(PartialEq, Properties)]
pub struct TooltipProps {
    pub title: AttrValue,
    pub children: Html,

    #[prop_or_default]
    pub class: Classes,

    /// Position of the tooltip
    ///  default value is `TooltipPosition::Bottom`
    #[prop_or_default]
    pub position: TooltipPosition,

    /// Show arrow
    ///  default value is `true`
    #[prop_or(true)]
    pub arrow: bool,

    /// Offset in pixels
    ///  - first value is for x-axis
    ///  - second value is for y-axis
    #[prop_or(8.0)]
    pub offset: f64,
}

#[function_component]
pub fn Tooltip(
    TooltipProps {
        title,
        arrow,
        class,
        offset,
        position,
        children,
    }: &TooltipProps,
) -> Html {
    let visible = use_state(|| false);
    let coordinates = use_state(Coordinates::default);
    let tooltip_ref = use_node_ref();

    let common_tooltip_classes = classes!(
        "mm-custom-tooltip",
        "mm-absolute",
        "mm-bg-white",
        "mm-border",
        "mm-border-gray-300",
        "mm-p-2",
        "mm-z-10",
        "mm-rounded",
        "mm-shadow-md",
        "mm-transition-opacity",
        "mm-max-w-80",
        "mm-text-xs",
        // Hidden doesn't work because there is no element
        //  and we can't calculate it's width and height
        //  so the values will be `0`, `0` and the position
        //  of the element will be places incorrectly
        // (!*visible).then_some(Some("hidden")),
        (!*visible).then_some(Some("mm-invisible")),
        (!*visible).then_some(Some("mm-opacity-0")),
        (*visible).then_some(Some("mm-opacity-100")),
    );

    let common_arrow_classes = classes!(
        "before:mm-content-['']",
        "before:mm-absolute",
        "before:mm-w-0",
        "before:mm-h-0",
        "before:mm-border-solid",
        "before:mm-border-l-transparent",
        "before:mm-border-l-4",
        "before:mm-border-r-transparent",
        "before:mm-border-r-4",
        "before:mm-border-b-gray-low-400",
        format!("before:mm-border-b-[{TOOLTIP_SIZE}px]")
    );

    let specific_arrow_classes = match &position {
        TooltipPosition::Top => vec![
            "before:-mm-bottom-2",
            "before:-mm-left-1/2",
            "before:-mm-translate-x-1/2",
            "before:mm-rotate-180",
        ],
        TooltipPosition::Bottom => vec![
            "before:-mm-top-2",
            "before:mm-left-1/2",
            "before:-mm-translate-x-1/2",
        ],

        // Has a top centration mismatch
        TooltipPosition::Right => vec![
            "before:mm-top-[calc(44%)]",
            "before:-mm-left-1",
            "before:-mm-translate-x-1/2",
            "before:-mm-rotate-90",
        ],
        // Has a top centration mismatch
        TooltipPosition::Left => vec![
            "before:mm-top-[calc(44%)]",
            "before:-mm-right-3",
            "before:-mm-translate-x-1/2",
            "before:mm-rotate-90",
        ],
    };

    let final_classes = classes!(
        common_tooltip_classes,
        &arrow.then_some(common_arrow_classes),
        &arrow.then_some(specific_arrow_classes),
    );

    let on_mouse_enter = {
        let visible = visible.clone();
        let coordinates = coordinates.clone();
        let tooltip_ref = tooltip_ref.clone();
        let position = position.clone();
        let offset = *offset;
        let arrow = *arrow;

        Callback::from(move |event: MouseEvent| {
            let html_element = event.target_unchecked_into::<HtmlElement>();

            let rect = html_element.get_bounding_client_rect();

            let res = tooltip_ref.cast::<HtmlElement>();

            if let Some(tooltip_element) = res {
                let tooltip_rect = tooltip_element.get_bounding_client_rect();

                let tooltip_coordinates = match &position {
                    TooltipPosition::Top => {
                        let top = rect.top() - tooltip_rect.height() - offset;
                        let top = if arrow { top - TOOLTIP_SIZE } else { top };

                        Coordinates {
                            top,
                            left: rect.left() + rect.width() / 2.0 - tooltip_rect.width() / 2.0,
                        }
                    }
                    TooltipPosition::Bottom => {
                        let top = rect.bottom() + offset;
                        let top = if arrow { top + TOOLTIP_SIZE } else { top };

                        Coordinates {
                            top,
                            left: rect.left() + rect.width() / 2.0 - tooltip_rect.width() / 2.0,
                        }
                    }
                    TooltipPosition::Right => {
                        let left = rect.right() + offset;
                        let left = if arrow { left + TOOLTIP_SIZE } else { left };

                        Coordinates {
                            top: rect.top() + rect.height() / 2.0 - tooltip_rect.height() / 2.0,
                            left,
                        }
                    }
                    TooltipPosition::Left => {
                        let left = rect.left() - tooltip_rect.width() - offset;
                        let left = if arrow { left - TOOLTIP_SIZE } else { left };

                        Coordinates {
                            top: rect.top() + rect.height() / 2.0 - tooltip_rect.height() / 2.0,
                            left,
                        }
                    }
                };

                visible.set(true);
                coordinates.set(tooltip_coordinates);
            }
        })
    };

    let on_mouse_leave = use_callback((), {
        let visible = visible.clone();

        move |_event, _| {
            // We mustn't erase the tooltip coordinates
            //  otherwise the tooltip will be placed incorrectly
            //  during hide animation
            visible.set(false);
        }
    });

    html! {
        <span onmouseenter={on_mouse_enter} onmouseleave={on_mouse_leave} class={class.clone()}>
            <div
                class={final_classes}
                style={format!("top: {top}px; left: {left}px;", top = coordinates.top, left = coordinates.left)}
                ref={tooltip_ref}
            >
                { title.clone() }
            </div>
            { children.clone() }
        </span>
    }
}
