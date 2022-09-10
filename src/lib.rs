use game::Game;
use js_sys::Function;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};
use wasm_react::{
    c, export_components, h,
    hooks::{use_callback, use_effect, use_js_ref, use_state, Deps},
    Component,
};
use web_sys::{console, window, Element, HtmlElement, KeyboardEvent};

mod cell;
mod collidable;
mod direction;
mod food;
mod game;
mod position;
mod snake;

pub struct App;

impl TryFrom<JsValue> for App {
    type Error = JsValue;

    fn try_from(_: JsValue) -> Result<Self, Self::Error> {
        // not sure what to do in here? What is this for?
        // seems to be getting state back from the JS side of react?
        Ok(App)
    }
}

impl Component for App {
    fn render(&self) -> wasm_react::VNode {
        let game = use_state(|| Game::new(10));
        let speed = use_state(|| 200);
        let element_container = use_js_ref::<Element>(None);

        use_effect(
            {
                // Auto focus our container

                let element_container = element_container.clone();

                move || {
                    element_container
                        .current()
                        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
                        .map(|element| element.focus().ok());

                    || ()
                }
            },
            Deps::none(),
        );

        use_effect(
            {
                let game = game.clone();
                let speed = *speed.value();

                move || {
                    let tick_closure = Closure::new({
                        let mut game = game.clone();

                        move || {
                            game.set(|mut game| {
                                game.handle_tick();
                                game
                            });
                        }
                    });

                    let handle = window()
                        .unwrap_throw()
                        .set_interval_with_callback_and_timeout_and_arguments_0(
                            tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                            speed,
                        )
                        .unwrap_throw();

                    move || {
                        drop(tick_closure);
                        window().unwrap_throw().clear_interval_with_handle(handle);
                    }
                }
            },
            Deps::some(*speed.value()),
        );

        let handle_key_down = use_callback(
            {
                let mut game = game.clone();

                move |evt: KeyboardEvent| {
                    let key_code = evt.code();
                    console::log_1(&format!("key pressed: {}", key_code).into());

                    game.set(|mut game| {
                        game.handle_key_press(key_code);
                        game
                    })
                }
            },
            Deps::none(),
        );

        h!(div)
            .id("app")
            .ref_container(&element_container)
            .tabindex(0)
            .on_keydown(&handle_key_down)
            .build(c![game.value().render()])
    }
}

export_components! { App }
