CREATE TABLE threads(
    id INTEGER PRIMARY KEY AUTO_INCREMENT NOT NULL,
    title TEXT NOT NULL,
    text TEXT NOT NULL,
    date DATETIME NOT NULL,
    author_id INTEGER NOT NULL,
    likes INTEGER NOT NULL,
    dislikes INTEGER NOT NULL,

    FOREIGN KEY (author_id) REFERENCES users (id)
);

CREATE TABLE posts(
    id INTEGER PRIMARY KEY AUTO_INCREMENT NOT NULL,
    title TEXT NOT NULL,
    text TEXT NOT NULL,
    date DATETIME NOT NULL,
    author_id INTEGER NOT NULL,
    thread_id INTEGER NOT NULL,
    likes INTEGER NOT NULL,
    dislikes INTEGER NOT NULL,

    FOREIGN KEY (author_id) REFERENCES users (id),
    FOREIGN KEY (thread_id) REFERENCES threads (id)
);
