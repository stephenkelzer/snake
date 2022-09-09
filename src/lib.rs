mod direction;
mod game;
mod position;
mod random;
mod snake;

use std::{cell::RefCell, rc::Rc};

use game::Game;
use js_sys::Function;
use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
use web_sys::{console, window, HtmlDivElement, HtmlElement, KeyboardEvent};

thread_local! {
    static GAME: Rc<RefCell<Game>> = Rc::new(RefCell::new(Game::new(15)));

    static HANDLE_TICK: Closure<dyn FnMut()> = Closure::wrap(Box::new({
        || {
            GAME.with(|game| game.borrow_mut().tick());
            render();
        }
      }) as Box<dyn FnMut()>);

    static HANDLE_KEYDOWN: Closure<dyn FnMut(KeyboardEvent)> = Closure::wrap(Box::new({
        |event: KeyboardEvent| {
            GAME.with(|game| {
                game.borrow_mut().handle_key_press(event.key());
            });
        }
    }) as Box<dyn FnMut(KeyboardEvent)>)
}

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"Starting...".into());

    HANDLE_TICK.with(|handle_tick| {
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                handle_tick.as_ref().dyn_ref::<Function>().unwrap_throw(),
                200,
            )
            .unwrap_throw()
    });

    HANDLE_KEYDOWN.with(|handle_keydown| {
        window()
            .unwrap_throw()
            .add_event_listener_with_callback(
                "keydown",
                handle_keydown.as_ref().dyn_ref::<Function>().unwrap_throw(),
            )
            .unwrap_throw();
    });

    render();
}

pub fn render() {
    GAME.with(|game| {
        let game = game.borrow();

        let document = window().unwrap_throw().document().unwrap_throw();

        let root_container = document
            .get_element_by_id("root")
            .unwrap_throw()
            .dyn_into::<HtmlElement>()
            .unwrap_throw();

        root_container.set_inner_html("");

        let width = game.width;
        let height = game.height;

        if game.finished {
            let game_over_container = document
                .create_element("div")
                .unwrap_throw()
                .dyn_into::<HtmlDivElement>()
                .unwrap_throw();

            game_over_container.set_inner_text("Game Over!");

            root_container
                .append_child(&game_over_container)
                .unwrap_throw();
            return;
        }

        let game_container = document
            .create_element("div")
            .unwrap_throw()
            .dyn_into::<HtmlDivElement>()
            .unwrap_throw();

        game_container.set_id("game-container");

        game_container
            .style()
            .set_property(
                "grid-template",
                &format!("repeat({}, auto) / repeat({}, auto)", width, height),
            )
            .unwrap_throw();

        root_container.append_child(&game_container).unwrap_throw();

        for y in 0..height {
            for x in 0..width {
                let pos = (x, y);
                let field_element = document
                    .create_element("div")
                    .unwrap_throw()
                    .dyn_into::<HtmlDivElement>()
                    .unwrap_throw();

                field_element.set_class_name("field");

                field_element.set_inner_text({
                    if pos == game.food {
                        "🍎"
                    } else if game.snake.positions.get(0) == Some(&pos) {
                        "❇️"
                    } else if game.snake.positions.contains(&pos) {
                        "🟩"
                    } else {
                        " "
                    }
                });

                game_container.append_child(&field_element).unwrap_throw();
            }
        }
    })
}
