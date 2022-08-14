use types::DirResponse;
use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

#[function_component(Home)]
fn home() -> Html {
    let backend_url: String = "http://localhost:8082".to_string();
    let response = Box::new(use_state(|| None));
    let error = Box::new(use_state(|| None));
    let endpoint = Box::new(format!(
        "{backend_url}/root",
        backend_url = backend_url,
    ));
    let retry = {
        let response = response.clone();
        let error = error.clone();
        let endpoint = endpoint.clone();

        Callback::from(move |_| {
            let response = response.clone();
            let error = error.clone();
            let endpoint = endpoint.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let fetched_response = Request::get(&endpoint).send().await;
                match fetched_response {
                    Ok(resp) => {
                        let json: Result<DirResponse, _> = resp.json().await;
                        match json {
                            Ok(f) => {
                                response.set(Some(f));
                            }
                            Err(e) => error.set(Some(e.to_string())),
                        }
                    }
                    Err(e) => error.set(Some(e.to_string())),
                }
            });
        })
    };

    match (*response).as_ref() {
        Some(resp) => html! {
            <div>
                <p>{ resp.name.clone() }</p>
            </div>
        },
        None => match (*error).as_ref() {
            Some(e) => {
                html! {
                    <>
                        {"error"} {e}
                        <button onclick={retry}>{"retry"}</button>
                    </>
                }
            }
            None => {
                html! {
                    <>
                        {"no error"}
                        <button onclick={retry}>{"call get"}</button>
                    </>
                }
            }
        },
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
