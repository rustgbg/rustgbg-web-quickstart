use chrono;
use crate::schema::examples;

#[derive(Queryable, Serialize)]
pub struct Example {
    pub id: String,
    pub value1: Option<String>,
    pub value2: Option<i32>
}

#[derive(Insertable, AsChangeset)]
#[table_name = "examples"]
pub struct NewExample {
    pub id: String,
    pub value1: String,
    pub value2: i32
}

