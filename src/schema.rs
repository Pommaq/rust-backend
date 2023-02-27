// @generated automatically by Diesel CLI.

diesel::table! {
    challenges (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    flags (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    challenges,
    flags,
);
