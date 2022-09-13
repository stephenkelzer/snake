use yew::{function_component, html, Properties};

use crate::components::game::Game;

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub version: String,
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    html! {
        <div id={"app"} version={ props.version.clone() } tabindex={0}>
            <Game />
        </div>
    }
}

// pub struct App {
//     version: String,
// }

// #[derive(Debug, Properties, PartialEq)]
// pub struct Props {
//     pub version: String,
// }

// impl Component for App {
//     type Message = ();

//     type Properties = Props;

//     fn create(_: &yew::Context<Self>) -> Self {
//         Self {
//             version: env!("CARGO_PKG_VERSION").to_string(),
//         }
//     }

//     fn view(&self, _: &yew::Context<Self>) -> yew::Html {
//         html!(
//             <div id={"id"} version={ self.version.clone() } tabindex={0}>
//                 <game::Game size={20} />
//             </div>
//         )
//     }
// }

// fn render(&self) -> wasm_react::VNode {
//     let game = use_state(|| Game::new(10));
//     let speed = use_state(|| 200);
//     let element_container = use_js_ref::<Element>(None);

//     use_effect(
//         {
//             // Auto focus our container

//             let element_container = element_container.clone();

//             move || {
//                 element_container
//                     .current()
//                     .and_then(|element| element.dyn_into::<HtmlElement>().ok())
//                     .map(|element| element.focus().ok());

//                 || ()
//             }
//         },
//         Deps::none(),
//     );

//     use_effect(
//         {
//             let game = game.clone();
//             let speed = *speed.value();

//             move || {
//                 let tick_closure = Closure::new({
//                     let mut game = game.clone();

//                     move || {
//                         game.set(|mut game| {
//                             game.handle_tick();
//                             game
//                         });
//                     }
//                 });

//                 let handle = window()
//                     .unwrap_throw()
//                     .set_interval_with_callback_and_timeout_and_arguments_0(
//                         tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
//                         speed,
//                     )
//                     .unwrap_throw();

//                 move || {
//                     drop(tick_closure);
//                     window().unwrap_throw().clear_interval_with_handle(handle);
//                 }
//             }
//         },
//         Deps::some(*speed.value()),
//     );

//     let handle_key_down = use_callback(
//         {
//             let mut game = game.clone();

//             move |evt: KeyboardEvent| {
//                 let key_code = &evt.code()[..];
//                 console::log_1(&format!("key pressed: {}", key_code).into());

//                 game.set(|mut game| {
//                     game.handle_key_press(key_code);
//                     game
//                 })
//             }
//         },
//         Deps::none(),
//     );

//     let handle_focus = use_callback(
//         {
//             let mut game = game.clone();

//             move |_| {
//                 game.set(|mut game| {
//                     game.unpause();
//                     game
//                 })
//             }
//         },
//         Deps::none(),
//     );

//     let handle_blur = use_callback(
//         {
//             let mut game = game.clone();

//             move |_| {
//                 game.set(|mut game| {
//                     game.pause();
//                     game
//                 })
//             }
//         },
//         Deps::none(),
//     );

//     h!(div)
//         .id("app")
//         .attr("app-version", &JsValue::from_str(CARGO_PKG_VERSION))
//         .ref_container(&element_container)
//         .tabindex(0)
//         .on_keydown(&handle_key_down)
//         .on_focus(&handle_focus)
//         .on_blur(&handle_blur)
//         .build(c![game.value().render()])
// }
