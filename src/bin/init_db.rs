use novstar::*;
use sqlx::{Connection, SqliteConnection};
use std::fs;
use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(short, long)]
    database: String,
    #[arg(short, long)]
    rawfile: String,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let log_file = fs::File::create(".log")?;

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();

    let Cli { database, rawfile } = Cli::parse();

    fs::File::create_new(&database)?;
    let data = Data::from_raw(&fs::read_to_string(&rawfile)?).await?;
    log::info!("Data inited");
    let mut con = SqliteConnection::connect(&database).await?;
    data.save_db(&mut con).await?;
    Ok(())
}
