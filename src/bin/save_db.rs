use northstar::*;
use sqlx::{Connection, SqliteConnection};
use std::fs;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut con = SqliteConnection::connect("sqlite://data.db").await?;
    let data = Data::from_db(&mut con, false).await?;
    fs::write("gui/data.bin", data.encode())?;
    Ok(())
}
