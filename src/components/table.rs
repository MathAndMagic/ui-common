use yew::prelude::*;
use yew_nested_router::{components::*, target::Target};

/// Renderer for a cell in a table.
pub struct CellRenderer<I>(Box<dyn Fn(&I) -> Html>);

/// Router for a cell in a table.
pub struct RowRouter<I, T>(Box<dyn Fn(&I) -> T>);

impl<I> CellRenderer<I> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&I) -> Html + 'static,
    {
        Self(Box::new(f))
    }
}

impl<I, T> RowRouter<I, T> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&I) -> T + 'static,
    {
        Self(Box::new(f))
    }
}

impl<I> PartialEq for CellRenderer<I>
where
    I: Clone + PartialEq,
{
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl<I, T> PartialEq for RowRouter<I, T>
where
    I: Clone + PartialEq,
    T: Target,
{
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

/// Column in a table.
#[derive(PartialEq)]
pub struct Column<I>
where
    I: Clone + PartialEq,
{
    pub title: Option<String>,
    pub cell: CellRenderer<I>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Variant {
    #[default]
    Classic,
    Separated,
}

#[derive(Properties, PartialEq)]
pub struct Props<I, T>
where
    I: Clone + PartialEq,
    T: Target,
{
    #[prop_or_default]
    pub variant: Variant,

    #[prop_or_default]
    pub columns: Vec<Column<I>>,

    #[prop_or_default]
    pub collection: Vec<I>,

    #[prop_or_default]
    pub cell_class: Classes,

    #[prop_or_default]
    pub display_header: bool,

    #[prop_or_default]
    pub router: Option<RowRouter<I, T>>,
}

#[function_component(Table)]
pub fn table<I, T>(props: &Props<I, T>) -> Html
where
    I: Clone + PartialEq,
    T: Target,
{
    let header = if props.display_header {
        let cells = props
            .columns
            .iter()
            .map(|column| {
                html! {
                    <th class="mm-text-sm mm-font-normal mm-text-left mm-px-4 mm-py-2">
                        if let Some(title) = &column.title {
                            { title.clone() }
                        }
                    </th>
                }
            })
            .collect::<Html>();

        html! {
            <thead>
                <tr class="mm-text-gray-low-50 dark:mm-text-gray-low-100">
                    {cells}
                </tr>
            </thead>
        }
    } else {
        html! {}
    };

    let last_column_idx = props.columns.len() - 1;

    let rows = props
        .collection
        .clone()
        .iter()
        .map(|item| {
            let cells = props
                .columns
                .iter()
                .enumerate()
                .map(|(i, column)| {
                    let border_classes = match props.variant {
                        Variant::Separated if i == 0 => {
                            classes!("mm-max-w-lg", "mm-rounded-l-lg", props.cell_class.clone(),)
                        }
                        Variant::Separated if i == last_column_idx => {
                            classes!("mm-max-w-lg", "mm-rounded-r-lg", props.cell_class.clone(),)
                        }
                        Variant::Separated => classes!("mm-max-w-lg", props.cell_class.clone(),),
                        Variant::Classic => props.cell_class.clone(),
                    };

                    let width_classes = match props.variant {
                        Variant::Classic if i == 0 => "mm-w-[90%] mm-min-w-64",
                        _ => "",
                    };

                    let class = classes!(
                        props.cell_class.clone(),
                        border_classes,
                        width_classes,
                        "mm-text-base"
                    );

                    html! {
                        <td {class}>{ (column.cell.0)(item) }</td>
                    }
                })
                .collect::<Html>();

            let border_classes = match props.variant {
                Variant::Separated => "",
                Variant::Classic => {
                    "mm-border-b mm-border-gray-high-500 dark:mm-border-gray-low-800"
                }
            };

            let class = classes!(
                "mm-bg-gray-high-300",
                "dark:mm-bg-gray-low-900",
                "mm-group",
                "hover:mm-bg-gray-high-200",
                "dark:hover:mm-bg-gray-low-800",
                "mm-transition-colors",
                "mm-duration-125",
                "mm-text-gray-low-800",
                "dark:mm-text-gray-high-200",
                border_classes,
            );

            if let Some(router) = &props.router {
                let to = (router.0)(item);

                let class = classes!(class, "mm-table-row");

                return html! {
                    <Link<T> {to} {class}>
                        {cells}
                    </Link<T>>
                };
            }

            html! {
                <tr {class}>
                    {cells}
                </tr>
            }
        })
        .collect::<Html>();

    let separation_classes = match props.variant {
        Variant::Classic => "",
        Variant::Separated => "mm-space-y-3 mm-border-spacing-y-3 mm-border-separate mm--mt-3",
    };

    let class = classes!("mm-w-full", separation_classes);

    html! {
        <div class="mm-overflow-x-auto">
            <table {class}>
                {header}
                <tbody>
                    {rows}
                </tbody>
            </table>
        </div>
    }
}
