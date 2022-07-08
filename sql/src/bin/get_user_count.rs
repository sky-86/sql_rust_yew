extern crate sql_project;
extern crate diesel;

use self::sql_project::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use sql_project::schema::tblusers::dsl::*;

    let connection = establish_connection();
    let results = tblusers
        .load::<User>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} user", results.len());
}
