table! {
    comments (id) {
        id -> Int4,
        user_id -> Int4,
        post_id -> Int4,
        body -> Text,
        creation_date -> Timestamptz,
    }
}

table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        slug -> Varchar,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        creation_date -> Timestamptz,
        edition_date -> Nullable<Timestamptz>,
        publication_date -> Nullable<Timestamptz>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

joinable!(comments -> posts (post_id));
joinable!(comments -> users (user_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    users,
);
