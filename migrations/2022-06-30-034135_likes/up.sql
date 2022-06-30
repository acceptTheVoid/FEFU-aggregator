CREATE TABLE thread_likes(
    id INTEGER PRIMARY KEY NOT NULL AUTO_INCREMENT,
    user_id INTEGER NOT NULL,
    liked_id INTEGER UNIQUE,
    disliked_id INTEGER UNIQUE,

    FOREIGN KEY (liked_id) REFERENCES threads (id),
    FOREIGN KEY (disliked_id) REFERENCES threads (id),
    FOREIGN KEY (user_ids) REFERENCES users (id)
);