use yew::prelude::*;
use stylist::yew::styled_component;

#[styled_component(HomePage)] 
pub fn view() -> Html {
    let box_style = css!(
        r#"
            display: flex;
            flex-direction: row;
            justify-content: flex-start;
            padding: 50px;
            margin: 20px;
            align-items: center;
            flex-flow: row wrap;
            @media (max-width: 900px) {
                flex-direction: column;
            }
        "#
    );
    let item_style = css!(
        r#"
            flex: 1;
            border: 1px solid;
            border-raduis: 2px;
            border-color: #8CA6DB;
            background: rgba(0,0,0,0.7);
            padding: 5px;
            margin: 10px;
            width: 200px;
            height: 250px;
            cursor: pointer;
            text-align: center;
            text-transform: uppercase;
            color: white;
        "#
    );


    let items: [&str; 10] = ["archive1", "archive2", "archive1", "archive2", "archive1", "archive2", "archive1", "archive2", "archive1", "archive2"];

    html! {
        <>
        <h1>{"Archives"}</h1>
            <div class={box_style}>
                {
                    items.into_iter().map(|item| {
                        html! {
                            <a href="#">
                                <div class={item_style.clone()}> {item}
                                </div>
                            </a>

                        }   
                    })
                    .collect::<Html>()
                }
            </div>
        </>
    }
}