use clap::Parser;
use novstar::*;
use sqlx::{Connection, SqliteConnection};
use std::fs;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(short, long)]
    database: String,
    #[arg(short, long)]
    out: String,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let Cli { database, out } = Cli::parse();

    env_logger::init();

    let mut con = SqliteConnection::connect(&database).await?;
    let data = Data::from_db(&mut con, true).await?;

    log::info!("data init");

    let iter = data.comments.iter();
    let res = tag_analyze(iter.collect());

    let mut res: Vec<_> = res.into_iter().filter(|(_, y)| *y >= 100).collect();

    res.sort_unstable_by(|x, y| x.1.cmp(&y.1).reverse());

    fs::write(out, serde_json::to_string_pretty(&res)?)?;
    Ok(())
}
