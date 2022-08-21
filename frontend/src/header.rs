use yew::prelude::*;
use stylist::yew::styled_component;

use crate::navigation::Navigation;

#[styled_component(Header)]
pub fn header() -> Html {
    html! {
        <nav class="navbar">
            <div id="trapezoid">
                    <Navigation /> 
            </div>
        </nav>
    }
}