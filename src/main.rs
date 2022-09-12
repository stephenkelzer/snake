fn main() {
    let app_version = env!("CARGO_PKG_VERSION").to_string();
    ui::run(app_version);
}
