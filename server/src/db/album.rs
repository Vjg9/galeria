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
