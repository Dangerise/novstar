use super::*;

const DATA_ASSET: Asset = asset!("data.bin");
static DATA: OnceLock<Data> = OnceLock::new();

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
    pub speed: f32,
    pub loaded: u32,
    pub full: u32,
}

pub async fn init_engine(mut state: Signal<State>) -> Engine<'static> {
    let url = resolve_asset(DATA_ASSET);
    tracing::info!("start require {}", url.as_str());

    let size = reqwest::get(url.clone())
        .await
        .unwrap()
        .content_length()
        .unwrap() as usize;

    tracing::info!("require file size {}", human_bytes(size as f64));

    state.write().as_downloading_mut().unwrap().full = size as u32;

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
        let mut s = state.write();
        let s = s.as_downloading_mut().unwrap();
        s.loaded = bytes.len() as u32;
        s.speed = (rec.len() as f32) / (last.elapsed().as_secs_f32());
        last = web_time::Instant::now();
    }

    tracing::info!("bytes required");

    let decom = zstd::decode_all(bytes.as_slice()).expect("zstd decode data");
    let (data, len): (Data, _) = bincode::decode_from_slice(&decom, bincode::config::standard())
        .expect("bincode decode data");
    assert_eq!(decom.len(), len);

    tracing::info!("decoded");

    let data: &'static Data = DATA.get_or_init(|| data);
    let engine = Engine::from_data(data);

    tracing::info!("future done");

    state.set(State::Ready(ReadyState {
        pattern: "".into(),
        searched_pattern: "".into(),
    }));

    engine
}

#[component]
pub fn download_page() -> Element {
    let state: Signal<State> = use_context();
    let state = state.read();
    let DownloadingState {
        speed,
        loaded,
        full,
    } = state.as_downloading().unwrap();
    let mut speed = *speed;
    let percent = (*loaded as f32) / (*full as f32);
    if speed > ((1u64 << 60u64) as f32) {
        speed = 0.;
    }
    let info = format!(
        "Speed : {}/s, {}/{}",
        human_bytes(speed),
        human_bytes(*loaded),
        human_bytes(*full)
    );
    rsx! {
        document::Stylesheet { href: asset!("assets/loading.css") }
        div { class: "loading",
            h1 { "Downloading" }
            div { class: "progress-bar-container",
                div { class: "progress-bar", style: "width: {percent*100.}%" }
            }
            p { class: "progress-info", "{info}" }
        }
    }
}
