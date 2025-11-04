mod data;
pub use data::Data;

mod engine;
pub use engine::{Engine, SearchResult};

pub const COMPRESS_LEVEL: i32 = 13;

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
