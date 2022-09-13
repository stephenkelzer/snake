use crate::game::cell::Cell;
use game::{element::Element, position::Position};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    pub table_layout: Vec<(usize, Vec<(Position, Option<Element>)>)>,
}

#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {
    let rows = props.table_layout.to_vec();

    html! {
        <div id={"game"}>
            {
                rows.into_iter().map(|(row, columns)| {
                    html! {
                        <div key={row} class={"row"} row-index={row.to_string()}>
                        {
                            columns.into_iter().map(|(position, cell)| {
                                    html!{<Cell key={position.column} element={cell} position={position} />}
                                }).collect::<Html>()
                            }
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
