use crate::Icon;
use gloo_utils::body;
use js_sys::Array;
use serde::Serialize;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::{
    use_click_away, use_event_with_window, use_swipe_with_options, UseSwipeDirection,
    UseSwipeOptions,
};

// How many pixels the user has to swipe down to close the modal
const MOBILE_SWIPE_THRESHOLD: i32 = 150;

#[derive(PartialEq, Properties)]
pub struct ModalTitleProps {
    pub children: Children,
}

#[function_component]
pub fn ModalTitle(ModalTitleProps { children }: &ModalTitleProps) -> Html {
    let context = use_context::<ModalContext>().expect("ModalTitle must be used inside a Modal");

    let on_close = {
        let on_close = context.on_close.clone();
        use_callback(on_close.clone(), move |_, _| {
            on_close.emit(());
        })
    };

    let wrapper_classes = if context.variant == ModalVariant::Bottom {
        classes!("mm-flex-col")
    } else {
        classes!("mm-flex-row", "mm-pt-6", "mm-pb-3", "mm-gap-4",)
    };

    let head_text_classes = match &context.variant {
        ModalVariant::Center => classes!("mm-text-left"),
        ModalVariant::Fullscreen => classes!("mm-text-left"),
        ModalVariant::Bottom => classes!("mm-text-center"),
    };

    let line = if context.variant == ModalVariant::Bottom {
        html! {
            <div
                class="mm-flex mm-pt-3 mm-pb-9 mm-justify-center mm-item-center mm-self-stretch mm-cursor-pointer"
            >
                <div class="mm-w-[72px] mm-h-[4px] mm-rounded-full mm-bg-gray-high-900 dark:mm-bg-gray-low-400"></div>
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <div class={classes!("mm-px-6", "mm-pb-3", "mm-flex", "mm-items-center", "mm-self-stretch", wrapper_classes)}>
            {line}
            <p
                class={classes!("mm-text-lg", "mm-font-bold", "mm-text-gray-low-800", "dark:mm-text-gray-high-200", "mm-grow", head_text_classes)}
            >
                {for children.iter() }
            </p>
            {
                if context.variant != ModalVariant::Bottom {
                    html! {
                        <button onclick={on_close} class="mm-shrink-0 mm-w-6 mm-h-6 mm-text-gray-low-100 dark:mm-text-gray-low-200 hover:mm-text-gray-low-800 dark:hover:mm-text-gray-high-200">
                            {Icon::X}
                        </button>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct ModalBodyProps {
    pub children: Children,
}

#[function_component]
pub fn ModalBody(props: &ModalBodyProps) -> Html {
    let context = use_context::<ModalContext>().expect("ModalBody must be used inside a Modal");
    let ModalBodyProps { children } = props;
    let node = use_node_ref();

    let classes = match &context.variant {
        ModalVariant::Center => classes!(),
        ModalVariant::Fullscreen => {
            classes!("mm-max-w-2xl", "mm-mx-auto", "mm-mb-auto")
        }
        ModalVariant::Bottom => classes!(),
    };

    let on_touch_prevent = {
        let node = node.clone();

        move |event: TouchEvent| {
            let node = node.cast::<HtmlElement>().unwrap();
            let scroll_y = node.scroll_top();

            if scroll_y > 0 {
                event.stop_propagation();
            }
        }
    };

    html! {
        <div
            class={classes!("mm-px-6", "mm-pt-3", "mm-pb-6", "mm-flex", "mm-flex-col", "mm-self-stretch", "mm-gap-4", "mm-overflow-y-scroll", "mm-text-lg", "mm-text-gray-low-400 ", "dark:mm-text-gray-high-700", "mm-grow", classes)}
            ref={node}
            ontouchstart={on_touch_prevent.clone()}
            ontouchmove={on_touch_prevent.clone()}
            ontouchend={on_touch_prevent}
        >
            {for children.iter() }
        </div>
    }
}
#[derive(PartialEq, Properties)]
pub struct ModalActionsProps {
    pub children: Children,
}

#[function_component]
pub fn ModalActions(props: &ModalActionsProps) -> Html {
    let context = use_context::<ModalContext>().expect("ModalTitle must be used inside a Modal");
    let ModalActionsProps { children } = props;

    let classes = match &context.variant {
        ModalVariant::Center => classes!(),
        ModalVariant::Fullscreen => {
            classes!("mm-max-w-2xl", "mm-mx-auto", "mm-mt-auto")
        }
        ModalVariant::Bottom => classes!(),
    };

    html! {
        <div class={classes!("mm-px-6", "mm-pt-3", "mm-pb-6", "mm-flex", "mm-justify-center", "mm-items-start", "mm-self-stretch", "mm-gap-4", "mm-w-full", classes)}>
            {for children.iter() }
        </div>
    }
}

#[derive(PartialEq, Default, Clone, Debug, Copy, Serialize)]
pub enum ModalVariant {
    #[default]
    Center,
    Fullscreen,
    Bottom,
}

#[derive(PartialEq, Properties)]
pub struct ModalProps {
    pub open: bool,
    pub on_close: Callback<()>,
    pub children: Children,

    #[prop_or_default]
    pub variant: ModalVariant,
}

#[derive(Clone, Debug, PartialEq)]
struct ModalContext {
    pub on_close: Callback<()>,
    pub variant: ModalVariant,
}

/// # Modal component
/// Is responsible for displaying a modal dialog
///  on top of the current page.
///
/// See [ModalProps](ModalProps) for the properties of this component.
///
/// ## Example (simplest option)
/// ```rust
/// use yew::prelude::*;
/// use ui_common::components::{Modal, ModalTitle, ModalBody, ModalActions};
///
/// #[function_component(TooltipExample)]
/// fn tooltip_example() -> Html {
///     let state = use_state(|| false);
///     let on_close = use_callback((), {
///         let state = state.clone();
///
///         move |_event, _| {
///             state.set(false);
///         }
///     });
///
///     html! {
///         <Modal open={*state} on_close={&on_close}>
///             <ModalTitle>{"Modal Title"}</ModalTitle>
///             <ModalBody>{"Modal Body"}</ModalBody>
///             <ModalActions>
///                 <button>{"Close"}</button>
///                 <button>{"Action"}</button>
///             </ModalActions>
///         </Modal>
///     }
/// }
/// ```
#[function_component]
pub fn Modal(
    ModalProps {
        open,
        variant,
        on_close,
        children,
    }: &ModalProps,
) -> Html {
    let dy_touch_coords = use_state(|| 0);

    let wrapper_classes = match &variant {
        ModalVariant::Center => classes!("mm-items-center", "mm-justify-center"),
        ModalVariant::Fullscreen => classes!("mm-items-center"),
        ModalVariant::Bottom => classes!("mm-items-end"),
    };

    let modal_classes = match &variant {
        ModalVariant::Center => classes!(
            "mm-rounded-lg",
            "mm-w-[calc(100%-24px)]",
            "mm-max-w-md",
            "mm-max-h-[calc(100vh-16px)]",
            "mm-mt-4",
            "mm-items-center"
        ),
        ModalVariant::Fullscreen => classes!("mm-w-full", "mm-h-full", "mm-items-center"),
        ModalVariant::Bottom => classes!(
            "mm-w-full",
            "mm-items-end",
            "mm-max-h-[65vh]",
            "mm-rounded-t-2xl",
            "mm-transition-transform",
            "mm-duration-75"
        ),
    };

    let node = use_node_ref();
    let context = ModalContext {
        on_close: on_close.clone(),
        variant: *variant,
    };

    let swipe = use_swipe_with_options(
        NodeRef::default(),
        UseSwipeOptions {
            onswipeend: Some(Box::new({
                let dy_touch_coords = dy_touch_coords.clone();
                let on_close = on_close.clone();

                move |_e, _direction| {
                    if *dy_touch_coords > MOBILE_SWIPE_THRESHOLD {
                        on_close.emit(());
                    }

                    dy_touch_coords.set(0);
                }
            })),
            ..Default::default()
        },
    );

    use_effect_with(swipe.length_y.clone(), {
        let dy_touch_coords = dy_touch_coords.clone();

        move |_| {
            if *swipe.direction == UseSwipeDirection::Down {
                dy_touch_coords.set(-*swipe.length_y);
            }
        }
    });

    use_event_with_window("keydown", {
        let on_close = on_close.clone();

        move |event: web_sys::KeyboardEvent| {
            // Close the modal when the Escape key is pressed
            if event.key() == "Escape" {
                on_close.emit(());
            }
        }
    });

    use_click_away(node.clone(), {
        let on_close = on_close.clone();

        move |_event: Event| {
            on_close.emit(());
        }
    });

    // Add the `overflow-hidden` class to the body
    //  when the modal is open to prevent scrolling
    use_effect_with(*open, {
        let open = *open;

        move |_| {
            if open {
                let classes = body().class_list();
                let classes_to_add = Array::new_with_length(1);
                classes_to_add.set(0, "mm-overflow-hidden".into());

                classes.add(&classes_to_add).expect("Class must be removed");
            } else {
                let classes = body().class_list();
                let classes_to_remove = Array::new_with_length(1);
                classes_to_remove.set(0, "mm-overflow-hidden".into());

                classes
                    .remove(&classes_to_remove)
                    .expect("Class must be removed");
            }
        }
    });

    if !open {
        return html! {};
    }

    return create_portal(
        html! {
            <ContextProvider<ModalContext> context={context}>
                <div
                    class={classes!("mm-fixed", "mm-left-0", "mm-top-0", "mm-w-screen", "mm-h-screen", "mm-bg-black", "mm-bg-opacity-80", "mm-flex", wrapper_classes)}
                >
                    <div
                        class={classes!("mm-bg-gray-high-200", "dark:mm-bg-gray-low-800", "mm-flex", "mm-flex-col", modal_classes)}
                        style={format!("transform: translateY({}px)", *dy_touch_coords)}
                        ref={node}
                    >
                        { for children.iter() }
                    </div>
                </div>
            </ContextProvider<ModalContext>>
        },
        body().into(),
    );
}
