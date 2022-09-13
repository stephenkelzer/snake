use gloo::{console::log, timers::callback::Interval};
use yew::{html, Component, Context, Html};

use crate::components::cell::Cell as CellComponent;
use crate::components::game_score::GameScore as GameScoreComponent;
use crate::components::game_status::GameStatus as GameStatusComponent;

use game::game::Game as GameEngine;

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
        log!("Updating game");
        match msg {
            Msg::Tick => {
                if !self.game.is_game_over() {
                    self.game.handle_tick();
                    // return true;
                }
            }
        }

        false
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        log!("Rending Game...");

        let rows = self.game.get_table_layout();

        html!(
            <div id={"game-wrapper"}>
                <GameStatusComponent is_playing={self.game.is_playing()} is_paused={self.game.is_paused()} is_game_over={self.game.is_game_over()} />
                <div id={"game"}>
                    {
                        rows.into_iter().map(|(row, columns)| {
                            html! {
                                <div key={row} class={"row"} row-index={row.to_string()}>
                                {
                                    columns.into_iter().map(|(position, cell)| {
                                            html!{<CellComponent key={position.column} cell={cell} position={position} />}
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
