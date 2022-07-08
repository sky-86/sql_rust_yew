extern crate sql_project;
extern crate diesel;

use self::sql_project::*;
use self::diesel::prelude::*;
use std::env::args;

fn main() {
    use sql_project::schema::tblusers::dsl::*;

    let target = args().nth(1).expect("Found no args");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(tblusers.filter(first.like(pattern)))
        .execute(&connection)
        .expect("Error deleting users");

    println!("Deleted {} users", num_deleted);
}
