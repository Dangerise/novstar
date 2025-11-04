use dioxus::logger::tracing::{self, error, Level};
use dioxus::prelude::*;
use human_bytes::human_bytes;

// async fn sleep() {
//     js_sys::Promise::new(|| web_sys::window().unwrap().set_timeout_with_str(""));
//     document::eval(include_str!("sleep.js")).await.unwrap();
// }

fn main() {
    dioxus::logger::init(Level::INFO).expect("logger init");
    std::panic::set_hook(Box::new(|info| error!("Panic Occured\n{}", info)));
    launch(app);
}

use scan::*;
use std::sync::OnceLock;

const DATA_ASSET: Asset = asset!("data.bin");
static DATA: OnceLock<Data> = OnceLock::new();

#[derive(Debug, Clone, PartialEq)]
struct SearchedPattern(String);

fn resolve_asset(s: impl ToString) -> reqwest::Url {
    let origin = web_sys::window().unwrap().origin();
    let url = reqwest::Url::parse(&origin)
        .unwrap()
        .join(s.to_string().as_str())
        .unwrap();
    url
}

#[derive(Debug, Clone, PartialEq)]
pub struct DownloadingState {
    speed: f32,
    loaded: u32,
    full: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReadyState {
    pattern: String,
    searched_pattern: String,
}

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

    let mut st = state.clone();
    let search_engine = use_resource(move || async move {
        let url = resolve_asset(DATA_ASSET);
        tracing::info!("start require {}", url.as_str());

        let size = reqwest::get(url.clone())
            .await
            .unwrap()
            .content_length()
            .unwrap() as usize;

        tracing::info!("require file size {}", human_bytes(size as f64));

        st.write().as_downloading_mut().unwrap().full = size as u32;

        let mut bytes: Vec<u8> = Vec::with_capacity(size);

        let mut stream = reqwest::get(url).await.unwrap().bytes_stream();

        // let mut last = web_time::Instant::now();
        // for _ in 1..=100 {
        //     sleep().await;
        //     let mut s = st.write();
        //     let s = s.as_downloading_mut().unwrap();
        //     let d = size as u32 / 100;
        //     s.loaded += d;
        //     s.speed = (d as f32) / (last.elapsed().as_secs_f32());
        //     last = web_time::Instant::now();
        // }

        let mut last = web_time::Instant::now();
        while let Some(item) = stream.next().await {
            let rec = item.unwrap();
            bytes.extend_from_slice(&*rec);
            let mut s = st.write();
            let s = s.as_downloading_mut().unwrap();
            s.loaded = bytes.len() as u32;
            s.speed = (rec.len() as f32) / (last.elapsed().as_secs_f32());
            last = web_time::Instant::now();
        }

        tracing::info!("bytes required");

        let decom = zstd::decode_all(bytes.as_slice()).expect("zstd decode data");
        let (data, len): (Data, _) =
            bincode::decode_from_slice(&decom, bincode::config::standard())
                .expect("bincode decode data");
        assert_eq!(decom.len(), len);

        tracing::info!("decoded");

        let data: &'static Data = DATA.get_or_init(|| data);
        let engine = SearchEngine::from_data(data);

        tracing::info!("future done");

        engine
    });

    use_context_provider(|| search_engine);
    let search_engine: Resource<SearchEngine> = use_context();

    let searched_pattern = use_signal(|| SearchedPattern(String::new()));
    use_context_provider(|| searched_pattern);

    if search_engine.read().is_some() {
        rsx! {
            h1 { "Search" }
            search_bar {}
            result_display {}
        }
    } else {
        let state = state.read();
        let DownloadingState {
            speed,
            loaded,
            full,
        } = state.as_downloading().unwrap();
        let percent = (*loaded as f32) / (*full as f32);
        let info = format!(
            "Speed : {}/s, {}/{}",
            human_bytes(*speed),
            human_bytes(*loaded),
            human_bytes(*full)
        );
        rsx! {
            document::Stylesheet { href: asset!("assets/loading.css") }
            div { class: "loading",
                h1 { "Downloading" }
                div { class: "progress-bar-container",
                    div {
                        class: "progress-bar",
                        style: "width: {percent*100.}%",
                    }
                }
                p { class: "progress-info", "{info}" }
            }
        }
    }
}

#[component]
fn search_bar() -> Element {
    const STYLE: Asset = asset!("assets/search.css");

    let mut pattern = use_signal(String::new);

    let pt = pattern.clone();
    let search = move || {
        let pattern = pt.read();
        let mut search_engine: Resource<SearchEngine> = use_context();
        let mut searched_pattern: Signal<SearchedPattern> = use_context();
        searched_pattern.set(SearchedPattern(pattern.cloned()));
        tracing::info!("search {}", pattern);
        let pattern: Vec<&str> = pattern.split_whitespace().collect();
        let mut write = search_engine.try_write().unwrap();
        (*write).as_mut().unwrap().search(&pattern).unwrap();
    };

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
            button { class: "search-button", onclick: move |_| search(), "搜索" }
        }
    }
}

#[component]
fn display_book(result: SearchResult) -> Element {
    let search_engine: Resource<SearchEngine> = use_context();
    let search_engine = search_engine.value();
    let rd = search_engine.read();
    let rd = (*rd).as_ref().unwrap();
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
    let search_engine: Resource<SearchEngine> = use_context();
    let search_engine = search_engine.value();
    let rd = search_engine.read();
    let rd = (*rd).as_ref().unwrap();
    let results = rd.results.as_slice();

    let searched_pattern: Signal<SearchedPattern> = use_context();
    let searched_pattern = searched_pattern.cloned().0;
    rsx! {
        document::Stylesheet { href: asset!("assets/result.css") }
        div { class: "search-results",
            p { class: "search-results__hint", "The results of \'{searched_pattern}\'" }
            for result in results.iter().map(|x| x.clone()) {
                div { class: "result-content",
                    display_book { result }
                }
            }
        }
    }
}
