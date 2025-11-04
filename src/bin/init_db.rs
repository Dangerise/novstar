use scan::*;
use sqlx::{Connection, SqliteConnection};
use std::fs;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    if fs::exists("data.db")? {
        fs::remove_file("data.db")?;
    }
    fs::File::create_new("data.db")?;
    let mut con = SqliteConnection::connect("sqlite://data.db").await?;
    init_db(&mut con).await?;
    Ok(())
}
