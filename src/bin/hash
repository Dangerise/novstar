fn main() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let raw = include_bytes!("../../data.bin").as_slice();
    let mut hasher = DefaultHasher::new();
    raw.hash(&mut hasher);
    let hs = hasher.finish();
    println!("raw len {} hash {}", raw.len(), hs);
}
