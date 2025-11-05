use novstar::*;
use sqlx::{Connection, SqliteConnection};
use std::fs;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let log_file = fs::File::create(".log")?;

    // env_logger::init();

    env_logger::Builder::new()
        .parse_default_env()
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();

    let db = "tmp.db";

    fs::File::create_new(db)?;
    let data = Data::from_raw(&fs::read_to_string("./tmp.txt")?).await?;
    log::info!("Data inited");
    let mut con = SqliteConnection::connect(db).await?;
    data.save_db(&mut con).await?;
    Ok(())
}
