pub const COMPRESS_LEVEL: i32 = 3;

#[cfg(feature = "native")]
use sqlx::{Connection, Row, SqliteConnection, sqlite::SqliteRow};

#[cfg(feature = "native")]
pub async fn init_db(con: &mut SqliteConnection) -> eyre::Result<()> {
    const DIV: &str = "\n\u{3000}\u{3000}\n\u{3000}\u{3000}\n\u{3000}\u{3000}";
    fn parse_book_name(text: &str) -> Option<&str> {
        let text = text.split_once("\n")?.0;
        let left = text.find("《")? + "《".len();
        let right = text.find("》")?;
        Some(&text[left..right])
    }
    use std::fs;
    let text = fs::read_to_string("data.txt")?;
    con.transaction(|tx| Box::pin(async move {
        sqlx::query("create table if not exists comments (id integer primary_key ,book_name text, content text)").execute(&mut **tx).await?;

        let mut comments = Vec::new();

        for elm in text.split(DIV) {
            if elm.contains("Time")             {
                continue;
            }

            let book_name =match parse_book_name(elm){
                Some(x)=>x,
                None=>"Other",
            };

            sqlx::query("insert into comments (book_name,content) values ($1,$2)").bind(book_name).bind(elm).execute(&mut **tx).await?;

            comments.push(Comment {
                book_name: book_name.into(),
                content: elm.into(),
            });

        }
        Ok::<(),eyre::Report>(())
    })).await?;

    Ok(())
}

use bincode::{Decode, Encode};
#[derive(Debug, Clone, Default, Encode, Decode, PartialEq)]
pub struct Comment {
    pub book_name: String,
    pub content: String,
}

#[derive(Debug, Clone, Default, Encode, Decode, PartialEq)]
pub struct Data {
    pub comments: Vec<Comment>,
}

impl Data {
    #[cfg(feature = "native")]
    pub async fn from_db(con: &mut SqliteConnection) -> eyre::Result<Self> {
        let comments = sqlx::query("select * from comments")
            .map(|row: SqliteRow| Comment {
                book_name: row.get("book_name"),
                content: row.get("content"),
            })
            .fetch_all(con)
            .await?;
        Ok(Data { comments })
    }
}

// #[derive(Debug, Clone)]
// pub struct Match {
//     pub from: usize,
//     pub score: i64,
// }

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SearchResult {
    Name(String),
    Id(usize),
}

#[derive(Debug, Clone)]
pub struct SearchEngine<'a> {
    pub data: &'a Data,
    pub map: HashMap<&'a str, Vec<usize>>,
    pub results: Vec<SearchResult>,
}

impl<'a> SearchEngine<'a> {
    pub fn from_data(data: &'a Data) -> Self {
        let mut map: HashMap<_, Vec<usize>> = HashMap::new();
        for (idx, Comment { book_name, .. }) in data.comments.iter().enumerate() {
            map.entry(book_name.as_str()).or_default().push(idx);
        }
        Self {
            data,
            map,
            results: Default::default(),
        }
    }
    pub fn get_book(&self, book_name: &str) -> Option<impl Iterator<Item = &str> + Clone> {
        let iter = self
            .map
            .get(book_name)?
            .iter()
            .map(|&x| self.data.comments[x].content.as_str());
        Some(iter)
    }
    pub fn search(&mut self, pattern: &[&str]) -> eyre::Result<()> {
        let Self { data, map, results } = self;

        results.clear();
        for (&book_name, list) in map.iter() {
            if book_name == "Other" {
                continue;
            }
            if pattern.iter().all(|&pat| {
                list.iter()
                    .map(|&x| &data.comments[x].content)
                    .any(|c| c.contains(pat))
            }) {
                results.push(SearchResult::Name(book_name.to_string()));
            }
        }

        let list = map
            .get("Other")
            .ok_or_else(|| eyre::eyre!("Other nou found"))?
            .iter()
            .map(|&x| (x, data.comments[x].content.as_str()));
        for (id, content) in list {
            if pattern.iter().all(|&pat| content.contains(pat)) {
                results.push(SearchResult::Id(id));
            }
        }
        Ok(())
    }
    // pub fn fuzzy_search(&mut self, pattern: &str) -> eyre::Result<()> {
    //     use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
    //     let matcher = SkimMatcherV2::default();

    //     let Self { comments, results } = self;
    //     results.clear();
    //     for (idx, Comment { book_name, content }) in comments.iter().enumerate() {
    //         let ret_book_name = matcher.fuzzy_match(book_name, pattern);
    //         let ret_content = matcher.fuzzy_match(content, pattern);
    //         if ret_book_name.is_some() || ret_content.is_some() {
    //             results.push(Match {
    //                 from: idx,
    //                 score: i64::max(
    //                     ret_book_name.unwrap_or(i64::MIN),
    //                     ret_content.unwrap_or(i64::MIN),
    //                 ),
    //             });
    //         }
    //     }

    //     results.sort_unstable_by(|x, y| x.score.cmp(&y.score));
    //     Ok(())
    // }
}
