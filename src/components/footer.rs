use yew::prelude::*;
use yew_nested_router::{components::*, target::Target};

use super::nav_link::NavLink;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props<T>
where
    T: Target,
{
    /// The links to display in the footer.
    #[prop_or_default]
    pub links: Vec<NavLink<T>>,

    /// The copyright content to display.
    #[prop_or_default]
    pub children: Children,
}

/// A footer component with 2 slots: `copyright` and `links`.
#[function_component]
pub fn Footer<T>(props: &Props<T>) -> Html
where
    T: Target + Default + 'static,
{
    html! {
        <footer class="mm-bg-gray-high-400 dark:mm-bg-gray-low-950 mm-text-gray-low-100 dark:mm-text-gray-low-200 mm-text-sm mm-p-6 mm-mt-auto">
            <div class="mm-container mm-mx-auto mm-flex mm-justify-between mm-px-6 mm-gap-y-6">
                if !props.links.is_empty() {
                    <div class="mm-flex-none md:mm-order-2">
                        <NavLinks<T> links={props.links.clone()} />
                    </div>
                }

                <div class="mm-flex-1 md:mm-order-1">
                    { for props.children.iter() }
                </div>
            </div>
        </footer>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct NavLinksProps<T>
where
    T: Target,
{
    /// The links to render.
    links: Vec<NavLink<T>>,
}

#[function_component]
fn NavLinks<T>(props: &NavLinksProps<T>) -> Html
where
    T: Target,
{
    let links = props.links.iter().map(|link| html! {
        <li class="mm-h-full">
            if let Some(route) = &link.route {
                <Link<T>
                    to={route.clone()}
                    class="mm-text-sm mm-inline-block mm-h-full mm-flex mm-items-center mm-gap-2 mm-justify-center mm-transition-colors mm-duration-125 mm-font-medium"
                    active="mm-text-gray-low-800 dark:mm-text-gray-high-200"
                    inactive="mm-text-gray-low-100 hover:mm-text-gray-low-400 dark:mm-text-gray-low-200 dark:hover:mm-text-gray-high-700"
                >
                    if let Some(icon) = link.icon {
                        <span class="mm-w-5 mm-inline-block mm-justify-center mm-items-center mm-flex mm-text-xl">{ icon }</span>
                    }

                    { link.text.to_string() }
                </Link<T>>
            } else {
                <a
                    href={link.href.clone()}
                    class="mm-text-sm mm-inline-block mm-h-full mm-flex mm-items-center mm-gap-2 mm-justify-center mm-transition-colors mm-duration-125 mm-font-medium mm-text-gray-low-100 hover:mm-text-gray-low-400 dark:mm-text-gray-low-200 dark:hover:mm-text-gray-high-700"
                >
                    if let Some(icon) = link.icon {
                        <span class="mm-w-5 mm-inline-block mm-justify-center mm-items-center mm-flex mm-text-xl">{ icon }</span>
                    }

                    { link.text.to_string() }
                </a>
            }
        </li>
    }).collect::<Html>();

    html! {
        <ul class="mm-flex mm-items-stretch mm-gap-6">
            { links }
        </ul>
    }
}
