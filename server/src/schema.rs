table! {
    comments (id) {
        id -> Int4,
        body -> Text,
        create_at -> Timestamp,
        post -> Nullable<Int4>,
        author -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        author -> Int4,
        create_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(comments -> posts (post));
joinable!(comments -> users (author));
joinable!(posts -> users (author));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    users,
);
