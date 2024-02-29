// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 50]
        email -> Nullable<Varchar>,
        #[max_length = 50]
        username -> Nullable<Varchar>,
    }
}
