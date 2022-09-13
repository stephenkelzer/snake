use crate::game::cell::Cell;
use game::{element::Element, position::Position};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub row_number: usize,
    pub columns: Vec<(Position, Option<Element>)>,
}

#[function_component(Row)]
pub fn row(props: &Props) -> Html {
    html! {
        <div key={props.row_number} class={"row"} row-index={props.row_number.to_string()}>
            {
                props.columns
                    .to_vec()
                    .into_iter()
                    .map(|(position, element)| {
                        html!{<Cell key={position.column} element={element} position={position} />}
                    })
                    .collect::<Html>()
            }
        </div>
    }
}
