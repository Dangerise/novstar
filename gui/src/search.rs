use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ReadyState {
    pub pattern: String,
    pub searched_pattern: String,
}

#[component]
pub fn search_bar() -> Element {
    const STYLE: Asset = asset!("assets/search.css");

    let search = move || {
        let mut state: Signal<State> = use_context();
        let mut state = state.write();
        let state = state.as_ready_mut().unwrap();
        let pattern = state.pattern.clone();
        state.searched_pattern = pattern.clone();
        let mut engine: Resource<Engine> = use_context();
        tracing::info!("search {}", pattern);
        let pattern: Vec<&str> = pattern.split_whitespace().collect();
        let mut write = engine.try_write().unwrap();
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
                    let mut state: Signal<State> = use_context();
                    state.write().as_ready_mut().unwrap().pattern = event.value();
                },
            }
            i { class: "fa fa-search search-icon" }
            button { class: "search-button", onclick: move |_| search(), "搜索" }
        }
    }
}

#[component]
pub fn display_book(result: String) -> Element {
    let engine: Resource<Engine> = use_context();
    let rd = engine.read();
    let rd = (*rd).as_ref().unwrap();
    let list=rd.get_book(&result).unwrap();
    let count = list.clone().count();
    rsx! {
        div { class: "result-item",
            h3 { class: "result-title", "{result}" }
            details {
                summary { "共{count}扫书" }
                for (idx , content) in list.enumerate() {
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
pub fn result_display() -> Element {
    let engine: Resource<Engine> = use_context();
    let engine = engine.value();
    let rd = engine.read();
    let rd = (*rd).as_ref().unwrap();
    let results = rd.results.as_slice();

    let state: Signal<State> = use_context();
    let searched_pattern = state.read().as_ready().unwrap().searched_pattern.clone();

    rsx! {
        document::Stylesheet { href: asset!("assets/result.css") }
        div { class: "search-results",
            if !searched_pattern.is_empty() {
                p { class: "search-results__hint", "The results of \'{searched_pattern}\'" }
            }
            for result in results.iter() {
                div { class: "result-content",
                    display_book { result }
                }
            }
        }
    }
}

#[component]
pub fn search_page() -> Element {
    rsx! {
        h1 { "Search" }
        search_bar {}
        result_display {}
    }
}
