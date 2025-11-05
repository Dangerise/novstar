CREATE TABLE comments (
    id INTEGER PRIMARY_KEY,
    book_name TEXT NOT NULL,
    content TEXT NOT NULL,
    words_cut BLOB
);