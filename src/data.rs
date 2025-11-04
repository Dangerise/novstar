use super::*;

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