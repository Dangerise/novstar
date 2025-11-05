mod data;
pub use data::Data;

mod engine;
pub use engine::{Engine, SearchResult};

mod tag;
pub use tag::tag_analyze; 

pub const COMPRESS_LEVEL: i32 = 13;

#[cfg(feature = "native")]
use sqlx::{Row, SqliteConnection, sqlite::SqliteRow};

use bincode::{Decode, Encode};
#[derive(Debug, Clone, Default, Encode, Decode, PartialEq)]
pub struct Comment {
    pub book_name: String,
    pub content: String,
    pub words_cut: Option<Vec<u8>>,
}
