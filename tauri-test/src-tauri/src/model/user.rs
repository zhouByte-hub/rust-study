use derive_getters::Getters;
use serde::{Deserialize, Serialize};


#[derive(Debug,Serialize, Deserialize, Getters)]
pub struct User{
    id: i32,
    name: String,
    age: i32,
    address: String
}

impl User {

    pub fn new(id: i32, name: String, age: i32, address: String) -> Self {
        Self {
            id,
            name,
            age,
            address
        }
    }
}