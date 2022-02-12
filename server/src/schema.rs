table! {
    comments (id) {
        id -> Int4,
        body -> Text,
        create_at -> Timestamp,
        post_id -> Nullable<Int4>,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        create_at -> Timestamp,
    }
}

joinable!(comments -> posts (post_id));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
);
