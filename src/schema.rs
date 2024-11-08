// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct UserRoleEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRoleEnum;

    user (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 5]
        role -> UserRoleEnum,
        created_at -> Timestamp,
    }
}
