use ui_common::components::Header;
use uuid::Uuid;
use yew::prelude::*;

use ui_common::{components::*, Icon};
use yew_nested_router::prelude::*;

#[derive(Target, Debug, Clone, PartialEq, Eq, Default)]
pub enum Route {
    #[target(index)]
    #[default]
    Index,

    Math,
    Magic,
}

#[function_component]
fn App() -> Html {
    html! {
        <Router<Route>>
            <Header<Route> logo={
                html! {
                    <svg width="78" height="24" viewBox="0 0 78 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M9.54849 17.9194C9.12568 18.8437 7.71474 21 3.75005 21C3.55114 21 3.36037 20.921 3.21972 20.7803C3.07907 20.6397 3.00005 20.4489 3.00005 20.25C3.00005 16.2853 5.1563 14.8744 6.08068 14.4516C6.1703 14.4107 6.26711 14.3879 6.36555 14.3844C6.464 14.3809 6.56216 14.3969 6.65444 14.4314C6.74672 14.4658 6.8313 14.5182 6.90336 14.5853C6.97543 14.6525 7.03355 14.7332 7.07443 14.8228C7.1153 14.9124 7.13812 15.0092 7.14159 15.1077C7.14505 15.2061 7.12909 15.3043 7.09461 15.3966C7.06014 15.4889 7.00783 15.5734 6.94067 15.6455C6.8735 15.7176 6.7928 15.7757 6.70318 15.8166C6.10036 16.0912 4.77005 16.9753 4.53568 19.4644C7.02474 19.23 7.91068 17.8997 8.18349 17.2969C8.22436 17.2072 8.28249 17.1265 8.35455 17.0594C8.42661 16.9922 8.5112 16.9399 8.60348 16.9054C8.69575 16.871 8.79392 16.855 8.89236 16.8585C8.99081 16.8619 9.08761 16.8847 9.17724 16.9256C9.26687 16.9665 9.34757 17.0246 9.41473 17.0967C9.48189 17.1687 9.5342 17.2533 9.56868 17.3456C9.60315 17.4379 9.61911 17.5361 9.61565 17.6345C9.61218 17.7329 9.58936 17.8297 9.54849 17.9194ZM20.986 4.41937C20.9638 4.05368 20.8085 3.70876 20.5494 3.4497C20.2903 3.19064 19.9454 3.03534 19.5797 3.01312C18.4004 2.94281 15.3863 3.05062 12.886 5.55093L8.25005 10.1906C8.18043 10.2604 8.09775 10.3157 8.00674 10.3535C7.91573 10.3913 7.81817 10.4108 7.71962 10.4109C7.5206 10.4111 7.32966 10.3322 7.1888 10.1916C7.04795 10.051 6.96872 9.86015 6.96854 9.66113C6.96837 9.4621 7.04726 9.27116 7.18786 9.13031L9.67786 6.63937C9.72999 6.58693 9.76546 6.52026 9.7798 6.44772C9.79414 6.37519 9.78671 6.30003 9.75846 6.2317C9.73021 6.16337 9.68238 6.10492 9.621 6.06369C9.55962 6.02247 9.48743 6.00031 9.41349 6H6.97036C6.77278 5.99895 6.57695 6.03721 6.39428 6.11253C6.21161 6.18785 6.04573 6.29874 5.9063 6.43875L2.69068 9.65625C2.49347 9.85332 2.35509 10.1014 2.29107 10.3728C2.22704 10.6441 2.23991 10.928 2.32821 11.1924C2.41652 11.4568 2.57678 11.6914 2.79101 11.8699C3.00523 12.0483 3.26494 12.1635 3.54099 12.2025L7.14755 12.7059L11.2922 16.8506L11.7957 20.4591C11.8344 20.7351 11.9496 20.9949 12.1281 21.2089C12.3067 21.4229 12.5416 21.5828 12.8063 21.6703C12.9605 21.7218 13.1219 21.7481 13.2844 21.7481C13.4812 21.7485 13.6761 21.7099 13.858 21.6346C14.0398 21.5593 14.2049 21.4488 14.3438 21.3094L17.5613 18.0937C17.7009 17.954 17.8115 17.7881 17.8868 17.6055C17.9621 17.4229 18.0006 17.2272 18.0001 17.0297V14.5866C17.9999 14.5124 17.9778 14.44 17.9366 14.3784C17.8953 14.3168 17.8367 14.2689 17.7682 14.2406C17.6996 14.2123 17.6243 14.2049 17.5516 14.2195C17.4789 14.234 17.4121 14.2697 17.3597 14.3222L14.8688 16.8122C14.7959 16.8851 14.7088 16.9422 14.6129 16.98C14.517 17.0178 14.4143 17.0353 14.3113 17.0316C14.2083 17.0279 14.1072 17.0031 14.0142 16.9585C13.9212 16.914 13.8384 16.8508 13.771 16.7728C13.648 16.624 13.5854 16.4343 13.5959 16.2415C13.6063 16.0487 13.6889 15.8669 13.8272 15.7322L18.4463 11.1131C20.9485 8.61187 21.0563 5.59781 20.986 4.4175V4.41937Z" fill="currentColor"/>
                        <path d="M28.0284 19.5L30.4432 4.95455H34.392L32.517 16.3182H38.3977L37.858 19.5H28.0284ZM51.3435 12.483C51.0783 14.0644 50.5527 15.3902 49.7668 16.4602C48.9808 17.5303 48.0267 18.3376 46.9045 18.8821C45.7824 19.4266 44.5821 19.6989 43.3037 19.6989C41.9401 19.6989 40.7563 19.4029 39.7526 18.8111C38.7488 18.2192 38.0172 17.3527 37.558 16.2116C37.0987 15.0705 37.0063 13.6761 37.281 12.0284C37.5366 10.447 38.0575 9.11648 38.8435 8.03693C39.6295 6.95739 40.5883 6.14062 41.7199 5.58665C42.8515 5.03267 44.0613 4.75568 45.3491 4.75568C46.7033 4.75568 47.8799 5.05398 48.879 5.65057C49.8828 6.24716 50.6119 7.12074 51.0665 8.27131C51.5258 9.42187 51.6181 10.8258 51.3435 12.483ZM47.3662 12.0284C47.4988 11.1951 47.5082 10.4943 47.3946 9.92614C47.281 9.35322 47.0419 8.92235 46.6773 8.63352C46.3127 8.33996 45.8226 8.19318 45.2071 8.19318C44.5063 8.19318 43.8861 8.36364 43.3463 8.70455C42.8065 9.04545 42.3591 9.53551 42.004 10.1747C41.6489 10.8139 41.4003 11.5833 41.2582 12.483C41.1067 13.3258 41.0925 14.0265 41.2156 14.5852C41.3435 15.1439 41.5944 15.563 41.9685 15.8423C42.3473 16.1217 42.8397 16.2614 43.4457 16.2614C44.137 16.2614 44.7478 16.0956 45.2781 15.7642C45.8132 15.4328 46.2559 14.9522 46.6062 14.3224C46.9614 13.6927 47.2147 12.928 47.3662 12.0284ZM60.2711 9.75568C60.2522 9.50473 60.2001 9.28456 60.1148 9.09517C60.0296 8.90104 59.9089 8.73769 59.7526 8.60511C59.5964 8.4678 59.4022 8.366 59.1702 8.29972C58.943 8.22869 58.6754 8.19318 58.3677 8.19318C57.7048 8.19318 57.1011 8.3518 56.5566 8.66903C56.0121 8.98627 55.5552 9.44318 55.1859 10.0398C54.8213 10.6364 54.5703 11.3561 54.433 12.1989C54.291 13.0511 54.2791 13.7803 54.3975 14.3864C54.5159 14.9924 54.7716 15.4564 55.1646 15.7784C55.5623 16.1004 56.0997 16.2614 56.7768 16.2614C57.3828 16.2614 57.906 16.1738 58.3464 15.9986C58.7867 15.8234 59.1395 15.5748 59.4046 15.2528C59.6698 14.9309 59.8355 14.5521 59.9018 14.1165L60.5836 14.1875H57.2597L57.7143 11.375H64.1631L63.8364 13.392C63.6186 14.7178 63.1499 15.8518 62.4302 16.794C61.7152 17.7315 60.8251 18.4512 59.7597 18.9531C58.6944 19.4503 57.5249 19.6989 56.2512 19.6989C54.8402 19.6989 53.6494 19.3982 52.6788 18.7969C51.7081 18.1955 51.0145 17.3385 50.5978 16.2259C50.1859 15.1132 50.1054 13.7898 50.3563 12.2557C50.5552 11.053 50.915 9.98769 51.4359 9.05966C51.9614 8.13163 52.603 7.34801 53.3606 6.70881C54.1182 6.06487 54.9539 5.57955 55.8677 5.25284C56.7815 4.9214 57.7285 4.75568 58.7086 4.75568C59.5656 4.75568 60.3445 4.87879 61.0452 5.125C61.746 5.36648 62.345 5.71212 62.8421 6.16193C63.3393 6.60701 63.711 7.13494 63.9572 7.74574C64.2081 8.35653 64.3099 9.02651 64.2626 9.75568H60.2711ZM77.1107 12.483C76.8455 14.0644 76.3199 15.3902 75.5339 16.4602C74.748 17.5303 73.7939 18.3376 72.6717 18.8821C71.5496 19.4266 70.3493 19.6989 69.0709 19.6989C67.7072 19.6989 66.5235 19.4029 65.5197 18.8111C64.516 18.2192 63.7844 17.3527 63.3251 16.2116C62.8659 15.0705 62.7735 13.6761 63.0482 12.0284C63.3038 10.447 63.8247 9.11648 64.6107 8.03693C65.3966 6.95739 66.3554 6.14062 67.4871 5.58665C68.6187 5.03267 69.8285 4.75568 71.1163 4.75568C72.4705 4.75568 73.6471 5.05398 74.6462 5.65057C75.65 6.24716 76.3791 7.12074 76.8337 8.27131C77.2929 9.42187 77.3853 10.8258 77.1107 12.483ZM73.1334 12.0284C73.266 11.1951 73.2754 10.4943 73.1618 9.92614C73.0482 9.35322 72.809 8.92235 72.4445 8.63352C72.0799 8.33996 71.5898 8.19318 70.9743 8.19318C70.2735 8.19318 69.6533 8.36364 69.1135 8.70455C68.5737 9.04545 68.1263 9.53551 67.7712 10.1747C67.416 10.8139 67.1675 11.5833 67.0254 12.483C66.8739 13.3258 66.8597 14.0265 66.9828 14.5852C67.1107 15.1439 67.3616 15.563 67.7357 15.8423C68.1144 16.1217 68.6069 16.2614 69.2129 16.2614C69.9042 16.2614 70.515 16.0956 71.0453 15.7642C71.5803 15.4328 72.0231 14.9522 72.3734 14.3224C72.7286 13.6927 72.9819 12.928 73.1334 12.0284Z" fill="currentColor"/>
                    </svg>
                }
            } links={vec![
                NavLink {
                    icon: Some(Icon::MATH_OPERATIONS),
                    route: Some(Route::Math),
                    text: "Math".to_string(),
                    ..Default::default()
                },
                NavLink {
                    icon: Some(Icon::MAGIC_WAND),
                    route: Some(Route::Magic),
                    text: "Magic".to_string(),
                    ..Default::default()
                },
            ]} right={
                html! {
                    <>
                        <Button<Route> color={ButtonColor::Blind} variant={ButtonVariant::Transparent} round={ButtonRound::Full} left_icon={Icon::MAGNIFYING_GLASS} />
                        <Button<Route> color={ButtonColor::Blind} variant={ButtonVariant::Transparent} round={ButtonRound::Full} left_icon={Icon::SUN} />
                        <Avatar<Route> name="Rinat Shay." variant={AvatarVariant::Letter} />
                    </>
                }
            } />

            <PageHeader text="Math and Magic UI components" buttons={vec![
                html! {
                    <Button<Route> size={ButtonSize::Large} left_icon={Some(Icon::PLUS)} text="Create Page" />
                },
            ]} />

            <div class="container mx-auto mt-4 px-2">
                <PageContent />
            </div>
        </Router<Route>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn PageContent() -> Html {
    let dropdown_id = format!("dropdown-{}", Uuid::new_v4());
    // let router = use_router().expect("no router found");

    // let menu_open = {
    //     let dropdown_id = dropdown_id.clone();

    //     use_callback(dropdown_id, |e: MouseEvent, dropdown_id| {
    //         e.prevent_default();
    //         e.stop_propagation();

    //         let document = web_sys::window()
    //             .expect("no window found")
    //             .document()
    //             .expect("no document found");

    //         // Remove all dropdowns
    //         let dropdowns = document.query_selector_all("[data-dropdown=true]").unwrap();
    //         for i in 0..dropdowns.length() {
    //             let dd = dropdowns.get(i).unwrap().dyn_into::<HtmlElement>().unwrap();
    //             dd.remove();
    //         }

    //         // Create dropdown element and append it to the body
    //         let dd = document.create_element("div").unwrap();
    //         dd.set_attribute("data-dropdown", "true").unwrap();
    //         dd.set_inner_html(&dropdown(&DropdownProps {
    //             id: dropdown_id.clone(),
    //         }));
    //         document.body().unwrap().append_child(&dd).unwrap();

    //         let rect = e
    //             .target()
    //             .expect("mouse event doesn't have a target")
    //             .dyn_into::<HtmlElement>()
    //             .expect("event target should be of type HtmlElement")
    //             .closest("button")
    //             .expect("no button found")
    //             .expect("no button found")
    //             .get_bounding_client_rect();

    //         // Position the dropdown
    //         let dd = document.get_element_by_id(&dropdown_id).unwrap();
    //         let dropdown_width = 240.0;

    //         dd.set_attribute(
    //             "style",
    //             &format!(
    //                 "top: {}px; left: {}px",
    //                 rect.bottom() + 4.0,
    //                 rect.right() - dropdown_width,
    //             ),
    //         )
    //         .expect("failed to set style");
    //     })
    // };

    // let menu_close = {
    //     let dropdown_id = dropdown_id.clone();

    //     use_callback(dropdown_id, |e: MouseEvent, dropdown_id| {
    //         e.prevent_default();
    //         e.stop_propagation();

    //         let document = web_sys::window()
    //             .expect("no window found")
    //             .document()
    //             .expect("no document found");

    //         let dropdown = document.get_element_by_id(&dropdown_id).unwrap();
    //         dropdown.class_list().toggle("hidden").unwrap();
    //     })
    // };

    let columns = vec![
        TableColumn {
            title: Some("Title".to_string()),
            cell: TableCellRenderer::new(|item: &String| html! { { (*item).clone() } }),
        },
        TableColumn {
            title: Some("Status".to_string()),
            cell: TableCellRenderer::new(|item: &String| html! { { (*item).clone() } }),
        },
        TableColumn {
            title: Some("Region".to_string()),
            cell: TableCellRenderer::new(|item: &String| html! { { (*item).clone() } }),
        },
        TableColumn {
            title: Some("Language".to_string()),
            cell: TableCellRenderer::new(|item: &String| html! { { (*item).clone() } }),
        },
        TableColumn {
            title: Some("Style".to_string()),
            cell: TableCellRenderer::new(|item: &String| html! { { (*item).clone() } }),
        },
        TableColumn {
            title: Some("Updated".to_string()),
            cell: TableCellRenderer::new(|item: &String| html! { { (*item).clone() } }),
        },
        TableColumn {
            title: None,
            cell: TableCellRenderer::new(move |_| {
                html! {
                    <Button<Route> left_icon={Some(Icon::DOTS_THREE_VERTICAL)} color={ButtonColor::Blind} variant={ButtonVariant::Transparent} />
                }
            }),
        },
    ];

    let collection = vec![
        "Hello".to_string(),
        "World".to_string(),
        "Foo".to_string(),
        "Bar".to_string(),
    ];

    let router = Some(TableCellRouter::new(|_| Route::Math));

    html! {
        <>
            <h2 class="mm-text-3xl mm-border-solid mm-border-b-gray-high-800 mm-border-b p-4">{"Table"}</h2>
            <Table<String, Route> {columns} display_header=true {router} {collection} variant={TableVariant::Classic} cell_class="px-4 py-3 align-middle" />

            <h2 class="mm-text-3xl mm-border-solid mm-border-b-gray-high-800 mm-border-b p-4">{"Tooltips"}</h2>
            <div class="mm-flex mm-flex-row p-2">
                <div class="mm-basis-1/4">
                    <Tooltip
                        title={"Let’s start with a Right placeholder"}
                        position={TooltipPosition::Right}
                        arrow={false}
                    >
                        <Button<Route>
                            color={ButtonColor::Primary}
                            variant={ButtonVariant::Solid}
                            text={"Right tooltip. No arrow"}
                        />
                    </Tooltip>
                </div>
                <div class="mm-basis-1/4">
                    <Tooltip
                        title={"Let’s start with a Bottom placeholder"}
                        position={TooltipPosition::Bottom}
                    >
                        <Button<Route>
                            color={ButtonColor::Primary}
                            variant={ButtonVariant::Solid}
                            text={"Bottom placeholder with an arrow"}
                        />
                    </Tooltip>
                </div>
                <div class="mm-basis-1/4">
                    <Tooltip
                        title={"Let’s start with a Left placeholder"}
                        position={TooltipPosition::Left}
                    >
                        <Button<Route>
                            color={ButtonColor::Primary}
                            variant={ButtonVariant::Solid}
                            text={"Left tooltip with an arrow"}
                        />
                    </Tooltip>
                </div>
                <div class="mm-basis-1/4">
                    <Tooltip
                        title={"Top tooltip"}
                        position={TooltipPosition::Top}
                        arrow={false}
                    >
                        <Button<Route>
                            color={ButtonColor::Primary}
                            variant={ButtonVariant::Solid}
                            text={"Top tooltip. No arrow"}
                        />
                    </Tooltip>
                </div>
            </div>
        </>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct DropdownProps {
    #[prop_or(format!("dropdown-{}", Uuid::new_v4()))]
    pub id: String,
}

fn dropdown(props: &DropdownProps) -> String {
    format!(
        r#"
        <div id={} class="absolute drop-shadow-md p-2 rounded-md border border-transparent-black-400 dark:border-transparent-white-400 bg-gray-high-100 dark:bg-gray-low-700 w-60 flex-rows text-gray-low-400 dark:text-gray-high-700">
            <div class="py-2 px-3">test</div>
            <div class="py-2 px-3">test 2</div>
        </div>
        "#,
        props.id
    )
}
