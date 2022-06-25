table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        text -> Text,
        date -> Datetime,
        author_id -> Integer,
        thread_id -> Integer,
        likes -> Integer,
        dislikes -> Integer,
    }
}

table! {
    threads (id) {
        id -> Integer,
        title -> Text,
        text -> Text,
        date -> Datetime,
        author_id -> Integer,
        likes -> Integer,
        dislikes -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}

joinable!(posts -> threads (thread_id));
joinable!(posts -> users (author_id));
joinable!(threads -> users (author_id));

allow_tables_to_appear_in_same_query!(
    posts,
    threads,
    users,
);
