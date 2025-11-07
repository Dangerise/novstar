use clap::{ArgAction, Parser};
use novstar::*;
use sqlx::{Connection, SqliteConnection};
use std::fs;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(short, long)]
    database: String,
    #[arg(short, long)]
    binary: String,
    #[arg(short, long,action=ArgAction::SetTrue)]
    words: bool,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let Cli {
        database,
        binary,
        words,
    } = Cli::parse();
    let mut con = SqliteConnection::connect(&database).await?;
    let data = Data::from_db(&mut con, words).await?;
    fs::write(binary, data.encode())?;
    Ok(())
}
