extern crate sql_project;
extern crate diesel;

use self::sql_project::*;
use self::diesel::prelude::*;

fn main() {
    use sql_project::schema::tblusers::dsl::*;

    let connection = establish_connection();
    let num_deleted = diesel::delete(tblusers)
        .execute(&connection)
        .expect("Error deleting users");

    println!("Deleted {} users", num_deleted);
}
