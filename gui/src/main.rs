use dioxus::logger::tracing::{error, Level};
use dioxus::prelude::*;

fn main() {
    dioxus::logger::init(Level::INFO).expect("logger failed to init");

    std::panic::set_hook(Box::new(|info| error!("Panic Occured\n{}", info)));

    launch(app);
}

use scan::*;
use std::sync::LazyLock;

static DATA: LazyLock<Data> = LazyLock::new(|| {
    let raw = include_bytes!("../../data.bin").as_slice();
    let decom = zstd::decode_all(raw).unwrap();
    let (data, len): (Data, _) =
        bincode::decode_from_slice(&decom, bincode::config::standard()).unwrap();
    assert!(decom.len() == len);
    data
});

#[component]
fn app() -> Element {
    let search_engine = use_signal(|| SearchEngine::from_data(&DATA));
    use_context_provider(|| search_engine);

    rsx! {
        h1 { "Search" }
        search_bar {}
        result_display {}
    }
}

#[component]
fn search_bar() -> Element {
    const STYLE: Asset = asset!("assets/search.css");

    let mut pattern = use_signal(String::new);

    rsx! {
        document::Stylesheet { href: STYLE }
        div { class: "search-container",
            input {
                r#type: "text",
                class: "search-input",
                placeholder: "输入书名",
                onchange: move |event| {
                    pattern.set(event.value());
                },
            }
            i { class: "fa fa-search search-icon" }
            button {
                class: "search-button",
                onclick: move |_| {
                    let mut search_engine: Signal<SearchEngine> = use_context();
                    let pattern = pattern.read();
                    let pattern: Vec<&str> = pattern.split_whitespace().collect();
                    search_engine.write().search(&pattern).unwrap();
                },
                "搜索"
            }
        }
    }
}

#[component]
fn display_book(result: SearchResult) -> Element {
    let search_engine: Signal<SearchEngine> = use_context();
    let rd = search_engine.read();
    let (name, list) = match &result {
        SearchResult::Id(id) => {
            let data = rd.data;
            ("Other", vec![data.comments[*id].content.as_str()])
        }
        SearchResult::Name(name) => (name.as_str(), rd.get_book(&name).unwrap().collect()),
    };
    let count = list.len();
    rsx! {
        div { class: "result-item",
            h3 { class: "result-title", "{name}" }
            details {
                summary { "共{count}扫书" }
                for (idx , content) in list.into_iter().enumerate() {
                    if idx > 0 {
                        hr {}
                    }
                    for line in content.lines() {
                        p { "{line}" }
                    }
                }
            }
        }
    }
}

#[component]
fn result_display() -> Element {
    let search_engine: Signal<SearchEngine> = use_context();
    let rd = search_engine.read();
    let results = rd.results.as_slice();
    rsx! {
        div { class: "search-results",
            for result in results.iter().map(|x| x.clone()) {
                div { class: "result-content",
                    display_book { result }
                }
            }
        }
    }
}
