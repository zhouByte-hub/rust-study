use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    pub poster: String,
    pub overview: String,
    pub release_date: i64,
    pub genres: Vec<String>,
}
