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
        topics -> Array<Int4>,
        create_at -> Timestamp,
        ups -> Int4,
        downs -> Int4,
    }
}

table! {
    posts_thumbs (id) {
        id -> Int4,
        post -> Int4,
        author -> Int4,
        voting -> Nullable<Bool>,
        create_at -> Timestamp,
    }
}

table! {
    topics (id) {
        id -> Int4,
        name -> Varchar,
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
joinable!(posts_thumbs -> posts (post));
joinable!(posts_thumbs -> users (author));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    posts_thumbs,
    topics,
    users,
);
