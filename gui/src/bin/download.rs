use dioxus::logger::tracing;
use dioxus::prelude::*;
fn main() {
    std::panic::set_hook(Box::new(|info| {
        let msg = format!("Panic from Rust\n{}", info);
        tracing::error!("{}", &msg);
        web_sys::window().unwrap().alert_with_message(&msg).unwrap();
    }));

    launch(app);
}

#[component]
fn app() -> Element {
    let data: Asset = asset!("data.bin");
    tracing::info!("asset path {}", data.to_string());

    rsx! {
        p { "finished" }
    }
}
