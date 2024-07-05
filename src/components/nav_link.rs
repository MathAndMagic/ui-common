use yew::prelude::*;
use yew_nested_router::target::Target;

use crate::Icon;

/// Navigation link.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct NavLink<T>
where
    T: Target,
{
    pub icon: Option<Icon>,

    /// The route to navigate to.
    ///
    /// Either this or `href` must be set.
    pub route: Option<T>,

    /// The `href` attribute for the anchor tag.
    ///
    /// Either this or `route` must be set.
    pub href: Option<String>,

    pub text: String,
    pub predicate: Option<Callback<T, bool>>,
}
