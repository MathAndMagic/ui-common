use yew::prelude::*;
use yew_nested_router::target::Target;

use crate::Icon;

/// Navigation link.
#[derive(Clone, Debug, PartialEq)]
pub struct NavLink<T>
where
    T: Target,
{
    pub icon: Option<Icon>,
    pub to: T,
    pub text: String,
    pub predicate: Option<Callback<T, bool>>,
}
