use scan::*;
use sqlx::{Connection, SqliteConnection};
use std::fs;
use std::io::{BufWriter, Write};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut con = SqliteConnection::connect("sqlite://data.db").await?;
    let data = Data::from_db(&mut con).await?;
    let engine = SearchEngine::from_data(&data);
    for (idx, &book_name) in engine.map.keys().enumerate() {
        let iter = engine.get_book(&book_name).unwrap();
        let book_name = String::from(book_name);
        let book_name = book_name.replace("/", "_");
        println!("Bookname {}", book_name);
        let file = fs::File::create(format!("./out/{}.txt", book_name))?;
        let mut writer = BufWriter::with_capacity(1 << 15, file);
        for content in iter {
            writer.write_all(content.as_bytes())?;
            writer.write_all("\n\n".as_bytes())?;
        }
    }
    Ok(())
}
