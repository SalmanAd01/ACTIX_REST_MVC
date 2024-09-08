// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        username -> Text,
        email -> Text,
        password -> Text,
        timestamp -> Timestamptz,
    }
}
