use dioxus::logger::tracing::{self, Level, error};
use dioxus::prelude::*;

mod download;
mod search;

use download::*;
use search::*;

use human_bytes::human_bytes;

fn main() {
    dioxus::logger::init(Level::INFO).expect("logger init");
    std::panic::set_hook(Box::new(|info| error!("Panic Occured\n{}", info)));
    launch(app);
}

use novstar::*;

#[derive(Debug, Clone, EnumExtract, PartialEq)]
pub enum State {
    Downloading(DownloadingState),
    Ready(ReadyState),
}

use enum_extract_macro::EnumExtract;
use futures_util::StreamExt;
#[component]
fn app() -> Element {
    let state = use_signal(|| {
        State::Downloading(DownloadingState {
            speed: 0.,
            loaded: 0,
            full: 0,
        })
    });
    use_context_provider(|| state);

    let st = state.clone();
    let engine = use_resource(move || init_engine(st));
    use_context_provider(|| engine);
    let engine: Resource<Engine> = use_context();

    rsx! {
        if engine.read().is_some() {
            search_page {}
        } else {
            download_page {}
        }
    }
}
