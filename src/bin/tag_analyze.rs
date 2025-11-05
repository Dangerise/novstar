use scan::*;
use sqlx::{Connection, SqliteConnection};
use std::fs;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut con = SqliteConnection::connect("sqlite://data.db").await?;
    let data = Data::from_db(&mut con, true).await?;

    let iter = data.comments.iter().take(300);
    let res = tag_analyze(iter.collect());

    // dbg!(&res);
    
    let mut res: Vec<_> = res.into_iter().filter(|(_, y)| *y >= 100).collect();

    res.sort_unstable_by(|x, y| x.1.cmp(&y.1).reverse());

    fs::write("key.json", serde_json::to_string_pretty(&res)?)?;
    Ok(())
}
