use yew::prelude::*;
use yew_router::prelude::*;
use stylist::yew::styled_component;

use crate::app::Route;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum URL {
    Local(Route),
}

#[derive(Properties, PartialEq)]
struct LocalRouteProps {
    route: Route,
    display: Html,
}

#[styled_component(LocalRoute)]
fn view_route_url(props: &LocalRouteProps) -> Html {
    html! {
        <Link<Route> to={props.route}>
            {props.display.clone()}
        </Link<Route>>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub url: URL,
    pub display: Html,
}

#[styled_component(NavLink)]
pub fn view(props: &Props) -> Html {
    html! {
        {
            match &props.url {
                URL::Local(route) => html! {
                    <LocalRoute
                    route={*route}
                    display={props.display.clone()}
                    />
                }
            }
        }
    }
}


