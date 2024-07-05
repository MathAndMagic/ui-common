use yew::prelude::*;
use yew_nested_router::{components::*, target::Target};

use super::nav_link::NavLink;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props<T>
where
    T: Target,
{
    /// The links to display in the header.
    #[prop_or_default]
    pub links: Vec<NavLink<T>>,

    /// The logo to display in the header.
    #[prop_or_default]
    pub logo: Children,

    /// Content for the right side of the header.
    #[prop_or_default]
    pub right: Children,
}

/// A header component with 3 slots: `logo`, `links`, and `right`.
#[function_component]
pub fn Header<T>(props: &Props<T>) -> Html
where
    T: Target + Default + 'static,
{
    html! {
        <header class="mm-bg-gray-high-400 dark:mm-bg-gray-low-950 dark:mm-border-b-gray-transparent-white-400 mm-border-b-transparent-black-400 mm-border-b-[0.5px] mm-text-gray-low-100 dark:mm-text-gray-low-200 mm-h-[52px]">
            <div class="mm-container mm-mx-auto mm-flex mm-px-6 mm-gap-x-4 sm:mm-gap-x-6 md:mm-gap-x-12 mm-h-full">
                <div class="mm-flex-none">
                    <div class="mm-h-full mm-flex mm-items-center">
                        <Link<T> to={T::default()} class="mm-block mm-h-full mm-flex mm-items-center">
                            { for props.logo.iter() }
                        </Link<T>>
                    </div>
                </div>
                <div class="mm-flex-grow">
                    if !props.links.is_empty() {
                        <NavLinks<T> links={props.links.clone()} />
                    }
                </div>
                <div class="mm-flex-none">
                    <div class="mm-flex mm-gap-x-2 sm:mm-gap-x-4 md:mm-gap-x-8 mm-h-full mm-items-center">
                        { for props.right.iter() }
                    </div>
                </div>
            </div>
        </header>
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
        <ul class="mm-flex mm-items-stretch mm-gap-1">
            { links }
        </ul>
    }
}
