use std::collections::HashMap;

use gloo::{console::log, timers::callback::Interval};
use yew::{html, Component, Context, Html};

use crate::components::cell::Cell as CellComponent;
use crate::components::game_score::GameScore as GameScoreComponent;
use crate::components::game_status::GameStatus as GameStatusComponent;
use game::{cell::Cell, game::Game as GameEngine, game_status::GameStatus};

#[derive(Debug)]
pub struct Game {
    game: GameEngine,
    _interval: Interval,
}

pub enum Msg {
    Tick,
}

impl Component for Game {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        log!("Creating game");

        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(200, move || callback.emit(()));

        Self {
            game: game::game::Game::new(),
            _interval: interval,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        // log!("Updating game");
        // match msg {
        //     Msg::Tick => {
        //         if self.game.status != GameStatus::GameOver {
        //             self.game.handle_tick();
        //             return true;
        //         }
        //     }
        // }

        false
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        let mut rows: HashMap<usize, Vec<(usize, Cell)>> = HashMap::new();

        self.game
            .get_all_positioned_cells()
            .into_iter()
            .for_each(|((_, y), cell)| {
                let row = rows.entry(y).or_insert(vec![]);
                row.push((y, cell));
            });

        html!(
            <div id={"game-wrapper"}>
                <GameStatusComponent status={self.game.status} />
                <div id={"game"}>
                    {
                        rows.into_iter().map(|(_, columns)| {
                            html! {
                                <div class={"row"}>
                                    {
                                        columns.into_iter().map(|(column, cell)| {
                                            html!{<CellComponent key={column} cell={cell} />}
                                        }).collect::<Html>()
                                    }
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <GameScoreComponent score={self.game.score()}  />
            </div>
        )
    }
}
