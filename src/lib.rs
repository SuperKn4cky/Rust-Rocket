pub mod models;
pub mod schema;

use crate::models::{NewUser, User};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use dotenvy::dotenv;
use schema::user::password;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(
    conn: &mut MysqlConnection,
    name: &str,
    email: &str,
    password: &str,
    role: models::Role,
) -> User {
    use crate::schema::user;

    let new_user = NewUser {
        name,
        email,
        password,
        role,
        created_at: NaiveDateTime,
    };

    conn.transaction(|conn| {
        diesel::insert_into(user::table)
            .values(&new_user)
            .execute(conn)?;

        user::table
            .order(user::id.desc())
            .select(User::as_select())
            .first(conn)
    })
    .expect("Error while saving post")
}
