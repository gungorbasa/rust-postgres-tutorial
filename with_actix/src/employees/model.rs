use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Employee {
    pub id: String,
    pub name: String,
    pub department: String,
    pub salary: i32,
    pub age: i32,
}
