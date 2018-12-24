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
        privacy -> Int4,
        content_warning -> Nullable<Varchar>,
        text -> Nullable<Varchar>,
        image_data -> Nullable<Json>,
        time -> Timestamptz,
    }
}

table! {
    posts_dests (id) {
        id -> Int4,
        post_id -> Int4,
        dest_id -> Int4,
    }
}

joinable!(posts -> accounts (src));

joinable!(posts_dests -> posts (post_id));

joinable!(posts_dests -> accounts (dest_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    posts,
    posts_dests,
);
