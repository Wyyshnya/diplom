use super::schema::students;
use diesel::{Insertable, Queryable};

#[derive(Insertable, Queryable)]
#[table_name="students"]
pub struct Student {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub age: i32
}

