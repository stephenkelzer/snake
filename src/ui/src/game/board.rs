use crate::game::row::Row;
use game::{element::Element, position::Position};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub table_layout: Vec<(usize, Vec<(Position, Option<Element>)>)>,
}

#[function_component(Board)]
pub fn board(props: &Props) -> Html {
    html! {
        <div id={"game"}>
            {
                props.table_layout
                    .to_vec()
                    .into_iter()
                    .map(|(row_number, columns)| {
                        html!(<Row {row_number} {columns} />)
                    })
                    .collect::<Html>()
            }
        </div>
    }
}
