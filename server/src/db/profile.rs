use sqlx::{
    Postgres,
    Pool
};
use crate::models::Profile;

// List profiles
pub async fn list(pool: &Pool<Postgres>) -> Result<Vec<Profile>, sqlx::Error> {
    let rows: Vec<(i32, String)> = sqlx::query_as(r#"SELECT * FROM profile"#)
        .fetch_all(pool)
        .await?;

    let profiles: Vec<Profile> = rows.iter().map(|row| {
        Profile {
            id: row.0,
            name: row.1.as_str().to_string()
        }
    })
    .collect();
   
    Ok(profiles)
}

// Add profile
pub async fn add(pool: &Pool<Postgres>, name: String) {
    let _profile = sqlx::query_as::<_, Profile>(
        "INSERT INTO profile (name) VALUES ($1) RETURNING id, name"
    )
        .bind(name)
        .fetch_one(pool)
        .await
        .unwrap();
}
