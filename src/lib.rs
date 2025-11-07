mod data;
pub use data::Data;

mod engine;
pub use engine::Engine;

#[cfg(feature = "native")]
mod tag;
#[cfg(feature = "native")]
pub use tag::tag_analyze;

mod sentence;

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

impl Comment {
    pub fn words(&self) -> Option<Vec<&str>> {
        let cut = self.words_cut.as_ref()?;
        let mut sum = 0;
        let mut words = Vec::with_capacity(cut.len());
        for &w in cut {
            words.push(&self.content[sum..sum + w as usize]);
            sum += w as usize;
        }
        Some(words)
    }
}
