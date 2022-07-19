table! {
    tasks (id) {
        id -> Varchar,
        content -> Text,
        user_id -> Varchar,
        done -> Bool,
    }
}

table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        pass -> Varchar,
    }
}

joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
