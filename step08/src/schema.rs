// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Nullable<Integer>,
        category -> Text,
        price -> Integer,
    }
}
