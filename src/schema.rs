table! {
    events (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

table! {
    feedback (id) {
        id -> Uuid,
        secret -> Varchar,
        body -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    events,
    feedback,
);
