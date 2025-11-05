use northstar::*;
use sqlx::{Connection, SqliteConnection};
use std::fs;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    env_logger::init();

    let mut con = SqliteConnection::connect("sqlite://data.db").await?;
    let data = Data::from_db(&mut con, true).await?;

    log::info!("data init");

    let mut out = String::new();
    for comment in data.comments {
        for word in comment.words().unwrap() {
            out.push_str(&format!("{word}|"));
        }
        out.push_str("\n\n");
    }

    fs::write("dump", out)?;
    Ok(())
}
