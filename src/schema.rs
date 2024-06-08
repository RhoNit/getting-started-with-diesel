// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        is_published -> Bool,
        likes -> Int4,
    }
}
