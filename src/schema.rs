table! {
    posts (id) {
        id -> Int4,
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

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
