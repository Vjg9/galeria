use serde::{Serialize, Deserialize};

// Profile model
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Profile {
    pub id: i32,
    pub name: String,
}

// Album model
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Album {
    pub id: i32,
    pub name: String,
    pub profile: i32
}

// Image model
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Image {
    pub id: i32,
    pub name: String,
    pub album: i32
}
