use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub text: AttrValue,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub buttons: Children,
}

#[function_component(PageHeader)]
pub fn page_header(props: &Props) -> Html {
    html! {
        <div class="mm-bg-gray-high-400 dark:mm-bg-gray-low-950 mm-py-6 mm-min-h-[100px] mm-border-b mm-border-b-gray-high-500 dark:mm-border-b-gray-low-800">
            <div class="mm-container mm-mx-auto mm-flex mm-px-6 mm-gap-x-4 sm:mm-gap-x-6 md:mm-gap-x-12 mm-h-full mm-items-center">
                <h1 class="mm-flex-1 mm-items-center mm-h-full mm-truncate mm-text-2xl mm-font-bold mm-text-gray-low-800 dark:mm-text-gray-high-200">
                    { props.text.clone() }
                </h1>
                if !props.buttons.is_empty() {
                    <div class="mm-flex-none">
                        { for props.buttons.iter() }
                    </div>
                }
            </div>
        </div>
    }
}
