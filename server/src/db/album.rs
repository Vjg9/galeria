use sqlx::{
    Postgres,
    Pool
};
use crate::models::Album;

// Add profile
pub async fn add(pool: &Pool<Postgres>, name: String, profile: i32) {
    let _album = sqlx::query_as::<_, Album>(
        "INSERT INTO album (name, profile) VALUES ($1, $2) RETURNING id, name, profile"
    )
        .bind(name)
        .bind(profile)
        .fetch_one(pool)
        .await
        .unwrap();
}

// Delete album 
pub async fn delete(pool: &Pool<Postgres>, id: i32) {
    sqlx::query(
        "DELETE FROM album WHERE id=$1"
    )
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}

// Update album
pub async fn update(pool: &Pool<Postgres>, id: i32, name: String, profile: i32) {
    sqlx::query(
        "UPDATE album SET name = $2, profile = $3 WHERE id = $1"
    )
        .bind(id)
        .bind(name)
        .bind(profile)
        .execute(pool)
        .await
        .unwrap();
}
