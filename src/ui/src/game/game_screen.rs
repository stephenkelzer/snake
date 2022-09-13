use gloo::events::EventListener;
use gloo::timers::callback::Interval;
use gloo::utils::document;
use wasm_bindgen::JsCast;
use yew::html::onkeydown::Event;
use yew::{html, Component, Context, KeyboardEvent};

use crate::game::board::Board;
use crate::game::footer::Footer;
use crate::game::header::Header;

use game::game::Game;

#[derive(Debug)]
pub struct GameScreen {
    game: Game,
    _interval: Interval,
    _key_down_listener: EventListener,
}

pub enum Msg {
    Tick,
    KeyDownEvent(KeyboardEvent),
}

impl Component for GameScreen {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let tick_callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(200, move || tick_callback.emit(()));

        let key_down_callback = ctx.link().callback(|e: Event| Msg::KeyDownEvent(e));
        let key_down_listener = EventListener::new(&document(), "keydown", move |e| {
            key_down_callback.emit(e.clone().unchecked_into::<KeyboardEvent>())
        });

        Self {
            game: Game::new(),
            _interval: interval,
            _key_down_listener: key_down_listener,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                if !self.game.is_game_over() {
                    self.game.handle_tick();
                    return true;
                }
            }
            Msg::KeyDownEvent(keyboard_event) => {
                self.game.handle_key_press(keyboard_event.code());
            }
        }

        false
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        let title: String;
        let subtitle: String;

        if self.game.is_playing() {
            title = "Playing".to_string();
            subtitle = "Use the arrow keys (or 'WASD') to move the snake.".to_string();
        } else if self.game.is_paused() {
            title = "Paused".to_string();
            subtitle = "Press 'space' to resume...".to_string();
        } else if self.game.is_game_over() {
            title = "Game Over!".to_string();
            subtitle = "Press 'space' to play again!".to_string();
        } else {
            panic!("Unknown game state")
        }

        html!(
            <div id={"game-wrapper"}>
                <Header {title} {subtitle} />
                <Board table_layout={self.game.get_table_layout()} />
                <Footer score={self.game.score()} />
            </div>
        )
    }
}
