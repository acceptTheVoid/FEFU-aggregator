table! {
    posts (id) {
        id -> Bigint,
        text -> Text,
        group_id -> Bigint,
    }
}

table! {
    public (id) {
        id -> Bigint,
        tag -> Text,
    }
}

joinable!(posts -> public (group_id));

allow_tables_to_appear_in_same_query!(
    posts,
    public,
);
