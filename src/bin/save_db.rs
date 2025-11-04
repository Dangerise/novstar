use scan::*;
use sqlx::{Connection, SqliteConnection};
use std::fs;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut con = SqliteConnection::connect("sqlite://data.db").await?;

    let data = Data::from_db(&mut con).await?;

    let raw = bincode::encode_to_vec(data, bincode::config::standard())?;

    let com = zstd::encode_all(raw.as_slice(), COMPRESS_LEVEL).unwrap();

    fs::write("data.bin", com)?;
    Ok(())
}
