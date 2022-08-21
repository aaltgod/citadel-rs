use yew::prelude::*;
use stylist::yew::styled_component;

use crate::{
    components::{NavLink, URL}, app::Route
};

#[styled_component(Navigation)]
pub fn navigation() -> Html {
    let nav_links: [(URL, Html); 2] = [
        (URL::Local(Route::Home), html!{{"Home"}}),
        (URL::Local(Route::Upload), html!{{"Upload"}}),
    ];

    html! {
            {
                nav_links.into_iter().map(|(url, display)| {
                    html! {
                        <NavLink url={url} display={display} />
                    }
                })
                .collect::<Html>()
            }
    }
}