use sqlx::{
    Postgres,
    Pool
};
use crate::models::{Album, Image};

// Get album by name 
pub async fn get_name(pool: &Pool<Postgres>, name: String) -> Result<Option<Album>, sqlx::Error> {
    match sqlx::query_as::<_, Album>(r#"SELECT * FROM album WHERE name=$1"#)
        .bind(&name)
        .fetch_one(pool)
        .await
    {
       Ok(album) => return Ok(Some(album)) ,
       Err(_) => return Ok(None)
    }
}

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

// List album's images
pub async fn list_images(pool: &Pool<Postgres>, id: i32) -> Result<Vec<Image>, sqlx::Error>{
    let rows: Vec<(i32, String, i32)> = sqlx::query_as(r#"SELECT * FROM image WHERE album=$1"#)
        .bind(id)
        .fetch_all(pool)
        .await?;

    let images: Vec<Image> = rows.iter().map(|row| {
        Image {
            id: row.0,
            name: row.1.as_str().to_string(),
            album: row.2
        }
    })
    .collect();

    Ok(images)
}
