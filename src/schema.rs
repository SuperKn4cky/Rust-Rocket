// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        role -> Varchar,
        created_at -> Timestamp,
    }
}
