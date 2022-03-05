use crate::components::MainComponent;

mod components;
mod network;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting the app...");
    let app = yew::start_app::<MainComponent>();
    log::info!("App has started !  \\ö/ {app:?}");
}
