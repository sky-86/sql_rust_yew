pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn establish_connection() -> PgConnection {

    let database_url = "postgresql://dboperator:operatorpass123@localhost:5243/postgres";

    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{User, NewUser};

pub fn create_user<'a>(conn: &PgConnection, first: &'a str, last: &'a str) -> User {
    use schema::tblusers;

    let new_user = NewUser {
        first,
        last,
    };

    diesel::insert_into(tblusers::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}
