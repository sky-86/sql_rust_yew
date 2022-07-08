extern crate sql_project;
extern crate diesel;

use self::sql_project::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("First name:");
    let mut first = String::new();
    stdin().read_line(&mut first).unwrap();
    let first = &first[..(first.len() - 1)]; // Drop the newline character
    
    println!("Last name:");
    let mut last = String::new();
    stdin().read_line(&mut last).unwrap();
    let last = &last[..(last.len() - 1)]; // Drop the newline character


    let user = create_user(&connection, first, last);
    println!("Create user: {} {}, {}", user.first, user.last, user.id);
}
