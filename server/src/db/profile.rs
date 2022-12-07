use sqlx::{
    Postgres,
    Pool
};
use crate::models::{Profile, Album};

// Get profile by name 
pub async fn get_name(pool: &Pool<Postgres>, name: String) -> Result<Option<Profile>, sqlx::Error> {
    match sqlx::query_as::<_, Profile>(r#"SELECT * FROM profile WHERE name=$1"#)
        .bind(&name)
        .fetch_one(pool)
        .await
    {
       Ok(profile) => return Ok(Some(profile)) ,
       Err(_) => return Ok(None)
    }
}

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

// Delete profile 
pub async fn delete(pool: &Pool<Postgres>, id: i32) {
    sqlx::query(
        "DELETE FROM profile WHERE id=$1"
    )
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}

// Update profile
pub async fn update(pool: &Pool<Postgres>, id: i32, name: String) {
    sqlx::query(
        "UPDATE profile SET name = $2 WHERE id = $1"
    )
        .bind(id)
        .bind(name)
        .execute(pool)
        .await
        .unwrap();
}

// List profile's albums
pub async fn list_albums(pool: &Pool<Postgres>, id: i32) -> Result<Vec<Album>, sqlx::Error>{
    let rows: Vec<(i32, String, i32)> = sqlx::query_as(r#"SELECT * FROM album WHERE profile=$1"#)
        .bind(id)
        .fetch_all(pool)
        .await?;

    let albums: Vec<Album> = rows.iter().map(|row| {
        Album {
            id: row.0,
            name: row.1.as_str().to_string(),
            profile: row.2
        }
    })
    .collect();

    Ok(albums)
}
