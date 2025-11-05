use super::*;
use dashmap::DashMap;
use rayon::prelude::*;

const STOP: &[&str] = &[
    ",", "-", "/", "\n", " ", "_", "+", "-", "，", "，", "。", "：", "　　", "—", "　", "(", ")",
    "（", "）", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", ".", ":", "@", "·", "《", "》",
    "的", "地", "得", "着", "了", "过", "所", "和", "与", "及", "或", "但", "而", "若", "因",
    "所以", "因为", "于是", "在", "于", "从", "到", "对", "对于", "关于", "把", "被", "由", "我",
    "你", "他", "她", "它", "我们", "你们", "他们", "这", "那", "此", "其", "谁", "什么", "很",
    "非常", "太", "也", "又", "还", "都", "只", "就", "才", "呀", "啊", "呢", "吗", "吧", "一",
    "二", "三", "个", "只", "本", "上", "下", "左", "右", "前", "后", "里", "外", "以", "者", "之",
    "怎", "么", "、", "】", "【", "？", "?", "!", "！",
];

const NEGATIVE_FIX: &[&str] = &["没有", "没", "不", "少", "无"];

pub fn tag_analyze(all: Vec<&Comment>) -> DashMap<&str, u32> {
    let mut cap = 0;
    for s in all.iter().map(|c| &c.content) {
        cap += s.len() * 4;
    }

    let map: DashMap<&str, u32> = DashMap::with_capacity(cap);
    all.into_par_iter().for_each(|comment| {
        let Comment {
            content, words_cut, ..
        } = comment;
        let words_cut = words_cut.as_ref().unwrap();
        let mut left: usize = 0;
        for i in 0..words_cut.len() {
            left += words_cut[i] as usize;
            let mut right = left;
            for j in 0..5 {
                if i + j < words_cut.len() {
                    right = words_cut
                        .get(i + j + 1)
                        .map(|&x| x as usize + right)
                        .unwrap_or(content.len());
                    let selected = &content[left..right];
                    if STOP.iter().any(|x| selected.contains(*x)) || selected.chars().count() == 1 {
                        continue;
                    }
                    *map.entry(selected).or_default() += 1;
                }
            }
        }
    });
    map
}
