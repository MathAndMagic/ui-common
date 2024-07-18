use gloo_utils::window;
use web_sys::HtmlElement;
use yew::prelude::*;

// Describes the size of the tooltip arrow (based on the largest side)
const TOOLTIP_ARROW_SIZE: f64 = 8.0;

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
    /// Title of the tooltip
    pub title: AttrValue,

    /// Above which element the tooltip should be shown
    pub children: Html,

    /// Additional classes on top of the tooltip
    #[prop_or_default]
    pub class: Classes,

    /// Position of the tooltip
    ///  default value is `TooltipPosition::Right`
    #[prop_or_default]
    pub position: TooltipPosition,

    /// Show arrow
    ///  default value is `true`
    #[prop_or(true)]
    pub arrow: bool,

    /// Offset in pixels
    ///  - first value is for x-axis
    ///  - second value is for y-axis
    /// default is 8px
    #[prop_or(8.0)]
    pub offset: f64,
}

/// # Tooltip component
/// Tooltip component is responsible to show a tooltip
///  on any children element when the user hovers over it.
///
/// See [TooltipProps](TooltipProps) for more details.
///
/// ## Example (simplest option)
/// ```rust
/// use yew::prelude::*;
/// use ui_common::components::{Tooltip, TooltipPosition};
///
/// #[function_component(TooltipExample)]
/// fn tooltip_example() -> Html {
///     html! {
///         <Tooltip title={"Tooltip title"}>
///             <button>{"Hover me"}</button>
///         </Tooltip>
///     }
/// }
/// ```
///
/// ## Example (rich option)
/// ```rust
/// use yew::prelude::*;
/// use ui_common::components::{Tooltip, TooltipPosition};
///
/// #[function_component(TooltipExample)]
/// fn tooltip_example() -> Html {
///     html! {
///         <Tooltip
///             title={"Tooltip title"}
///             position={TooltipPosition::Bottom}
///             arrow={true}
///             offset={8.0}
///         >
///             <button>{"Hover me"}</button>
///         </Tooltip>
///     }
/// }
/// ```
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
        "mm-p-2",
        "mm-z-10",
        "mm-rounded",
        "mm-shadow-md",
        "mm-transition-opacity",
        "mm-max-w-80",
        "mm-text-xs",
        "mm-text-primary-100",
        "mm-bg-gray-low-700",
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
        "before:mm-border-l-[8px]",
        "before:mm-border-r-transparent",
        "before:mm-border-r-[8px]",
        "before:mm-border-b-gray-low-700",
        "before:mm-border-b-[8px]",
        format!("before:mm-border-b-[{TOOLTIP_ARROW_SIZE}px]")
    );

    let specific_arrow_classes = match &position {
        TooltipPosition::Bottom => classes!(
            "before:-mm-top-2",
            "before:mm-left-1/2",
            "before:-mm-translate-x-1/2",
        ),
        TooltipPosition::Right => classes!(
            "before:mm-top-[calc(50%-3px)]",
            "before:-mm-left-1",
            "before:-mm-translate-x-1/2",
            "before:-mm-rotate-90",
        ),
        TooltipPosition::Top => classes!(
            "before:-mm-bottom-2",
            "before:mm-left-1/2",
            "before:-mm-translate-x-1/2",
            "before:mm-rotate-180",
        ),
        TooltipPosition::Left => classes!(
            "before:mm-top-[calc(50%-3px)]",
            "before:-mm-right-[18px]",
            "before:-mm-translate-x-1/2",
            "before:mm-rotate-90",
        ),
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
            // HTML element on which the event was triggered
            let html_element = event.target_unchecked_into::<HtmlElement>();
            let html_element_rect = html_element.get_bounding_client_rect();

            let tooltip_element = tooltip_ref.cast::<HtmlElement>();

            if let Some(tooltip_element) = tooltip_element {
                let tooltip_rect = tooltip_element.get_bounding_client_rect();

                let tooltip_coordinates = match &position {
                    TooltipPosition::Top => {
                        // Get `y` coordinate based on `x1, y1` from top-left corner + window scroll
                        let top = html_element.offset_top() as f64;

                        // Subtract the tooltip height and add an offset
                        let top = top - tooltip_rect.height() - offset;

                        // If the arrow is enabled, we need to add the arrow size
                        let top = if arrow { top - TOOLTIP_ARROW_SIZE } else { top };

                        Coordinates {
                            top,
                            left: html_element_rect.left() + html_element_rect.width() / 2.0
                                - tooltip_rect.width() / 2.0,
                        }
                    }
                    TooltipPosition::Bottom => {
                        // Get `y` coordinate based on `x4, y4` from bottom-left corner + window scroll
                        let top = html_element_rect.bottom()
                            + window().scroll_y().expect("Must have window on the page")
                            + offset;

                        // If the arrow is enabled, we need to add the arrow size
                        let top = if arrow { top + TOOLTIP_ARROW_SIZE } else { top };

                        Coordinates {
                            top,
                            left: html_element_rect.left() + html_element_rect.width() / 2.0
                                - tooltip_rect.width() / 2.0,
                        }
                    }
                    TooltipPosition::Right => {
                        // Get `y` coordinate based on `x1, y1` from top-left corner + window scroll
                        let top = html_element.offset_top() as f64;
                        let top =
                            top + html_element_rect.height() / 2.0 - tooltip_rect.height() / 2.0;

                        let left = html_element_rect.right() + offset;
                        let left = if arrow {
                            left + TOOLTIP_ARROW_SIZE
                        } else {
                            left
                        };

                        Coordinates { top, left }
                    }
                    TooltipPosition::Left => {
                        // Get `y` coordinate based on `x1, y1` from top-left corner + window scroll
                        let top = html_element.offset_top() as f64;
                        let top =
                            top + html_element_rect.height() / 2.0 - tooltip_rect.height() / 2.0;

                        let left = html_element_rect.left() - tooltip_rect.width() - offset;
                        let left = if arrow {
                            left - TOOLTIP_ARROW_SIZE
                        } else {
                            left
                        };

                        Coordinates { top, left }
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
        <span onmouseenter={on_mouse_enter} onmouseleave={on_mouse_leave} class={classes!(class.clone(), "mm-inline-block")}>
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
