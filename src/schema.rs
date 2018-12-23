table! {
    accounts (id) {
        id -> Int4,
        url -> Varchar,
    }
}

table! {
    posts (id) {
        id -> Int4,
        src -> Int4,
        dest -> Int4,
        privacy -> Int4,
        content_warning -> Nullable<Varchar>,
        text -> Nullable<Varchar>,
        image_data -> Nullable<Json>,
        time -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    posts,
);
