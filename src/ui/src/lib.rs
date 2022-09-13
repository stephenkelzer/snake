mod app;
mod game;

pub fn run(app_version: String) {
    yew::start_app_with_props::<app::App>(app::Props {
        version: app_version,
    });
}
