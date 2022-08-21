use yew::prelude::*;
use stylist::yew::styled_component;

#[styled_component(HomePage)] 
pub fn view() -> Html {
    let box_style = css!(
        r#"
            display: flex;
            justify-content: flex-start;
            padding: 50px;
            margin: 20px;
        "#
    );

    let item_style = css!(
        r#"
            flex: 1;
            border: 1px solid;
            border-raduis: 2px;
            border-color: rgba(0,0,0,0.7);
            background: rgba(0,0,0,0.7);
            padding: 5px;
            margin: 10px;
            width: 200px;
            height: 250px;
            clickable: true;
        "#
    );

    let items: [&str; 2] = ["archive1", "archive2"];

    html! {
        <>
        <h1>{"Archives"}</h1>
            <div class={box_style}>
                {
                    items.into_iter().map(|item| {
                        html! {
                            <div class={item_style.clone()}> {item}
                                <a href="#"><span /> </a>
                            </div>
                        }   
                    })
                    .collect::<Html>()
                }
            </div>
        </>
    }
}