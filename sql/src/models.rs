#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub first: String,
    pub last: String,
}

use super::schema::tblusers;

#[derive(Insertable)]
#[table_name="tblusers"]
pub struct NewUser<'a> {
    pub first: &'a str,
    pub last: &'a str,
}
