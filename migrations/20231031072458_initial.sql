-- Add migration script here
CREATE TABLE books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT,
    author TEXT
);

INSERT INTO books (title, author) VALUES ('Phap luat dai cuong', 'Mac Nhu Hieu');
INSERT INTO books (title, author) VALUES ('Ki thuat lap trinh', 'Luu Anh Tuan');