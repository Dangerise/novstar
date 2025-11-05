use super::*;

#[derive(Debug, Clone, Default, Encode, Decode, PartialEq)]
pub struct Data {
    pub comments: Vec<Comment>,
}

impl Data {
    #[cfg(feature = "native")]
    pub async fn from_raw(text: &str) -> eyre::Result<Self> {
        const DIV: &str = "\n\u{3000}\u{3000}\n\u{3000}\u{3000}\n\u{3000}\u{3000}";
        fn parse_book_name(text: &str) -> Option<&str> {
            let text = text.split_once("\n")?.0;
            let left = text.find("《")? + "《".len();
            let right = text.find("》")?;
            Some(&text[left..right])
        }
        let mut comments = Vec::new();
        let mut no_specific_count = 0;
        for elm in text.split(DIV) {
            if elm.contains("Time") {
                continue;
            }

            let book_name = match parse_book_name(elm) {
                Some(x) => x.to_string(),
                None => {
                    no_specific_count += 1;
                    format!("NoSpecific{no_specific_count}")
                }
            };

            comments.push(Comment {
                book_name: book_name,
                content: elm.into(),
                words_cut: None,
            });
        }

        use rayon::prelude::*;
        use std::sync::Mutex;
        let len = comments.len();
        let start = 0;
        let count = Mutex::new(start);

        comments
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, comment)| {
                log::info!("{idx} started");
                let jb = jieba_rs::Jieba::new();
                let words = jb.cut(&comment.content, true);
                log::info!("{idx} cuted");
                let mut delta = Vec::with_capacity(words.len());
                delta.push(0);
                for i in 1..words.len() {
                    let mut d = words[i].as_ptr() as usize - words[i - 1].as_ptr() as usize;
                    while d >= u8::MAX as usize {
                        d -= u8::MAX as usize;
                        delta.push(u8::MAX);
                    }
                    delta.push(d as u8);
                }
                comment.words_cut = Some(delta);
                log::info!("{idx} done");

                let mut lock = count.lock().unwrap();
                *lock += 1;
                log::info!("{}/{}", *lock, len);
            });

        Ok(Data { comments })
    }

    #[cfg(feature = "native")]
    pub async fn save_db(&self, con: &mut SqliteConnection) -> eyre::Result<()> {
        use sqlx::Connection;

        let mut txn = con.begin().await?;

        let f = async {
            sqlx::query(include_str!("create_table.sql"))
                .execute(&mut *txn)
                .await?;

            for comment in &self.comments {
                let Comment {
                    book_name,
                    content,
                    words_cut,
                } = comment;
                if let Some(words_cut) = words_cut {
                    sqlx::query(include_str!("insert3.sql"))
                        .bind(book_name)
                        .bind(content)
                        .bind(words_cut)
                        .execute(&mut *txn)
                        .await?;
                } else {
                    sqlx::query(include_str!("insert2.sql"))
                        .bind(book_name)
                        .bind(content)
                        .execute(&mut *txn)
                        .await?;
                }
            }
            Ok::<(), eyre::Report>(())
        }
        .await;

        match f {
            Ok(()) => {
                txn.commit().await?;
                Ok(())
            }
            Err(err) => {
                txn.rollback().await?;
                Err(err)
            }
        }

        // for comment in &self.comments {
        //     let Comment {
        //         book_name,
        //         content,
        //         words_cut,
        //     } = comment;
        //     if let Some(words_cut) = words_cut {
        //         sqlx::query(include_str!("insert3.sql"))
        //             .bind(book_name)
        //             .bind(content)
        //             .bind(words_cut)
        //             .execute(&mut **txn)
        //             .await?;
        //     } else {
        //         sqlx::query(include_str!("insert2.sql"))
        //             .bind(book_name)
        //             .bind(content)
        //             .execute(&mut **txn)
        //             .await?;
        //     }
        // }
    }

    #[cfg(feature = "native")]
    pub async fn from_db(con: &mut SqliteConnection, with_cut: bool) -> eyre::Result<Self> {
        let comments = sqlx::query("select * from comments")
            .map(|row: SqliteRow| Comment {
                book_name: row.get("book_name"),
                content: row.get("content"),
                words_cut: with_cut.then(|| row.get("words_cut")),
            })
            .fetch_all(con)
            .await?;
        Ok(Data { comments })
    }

    pub fn encode(&self) -> Vec<u8> {
        let raw = bincode::encode_to_vec(self, bincode::config::standard()).unwrap();
        let com = zstd::encode_all(raw.as_slice(), COMPRESS_LEVEL).unwrap();
        com
    }
}
