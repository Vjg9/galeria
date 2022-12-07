use sqlx::{
    Postgres,
    Pool
};
use crate::models::Image;

// Get image by name 
pub async fn get_name(pool: &Pool<Postgres>, name: String) -> Result<Option<Image>, sqlx::Error> {
    match sqlx::query_as::<_, Image>(r#"SELECT * FROM image WHERE name=$1"#)
        .bind(&name)
        .fetch_one(pool)
        .await
    {
       Ok(image) => return Ok(Some(image)) ,
       Err(_) => return Ok(None)
    }
}

// Add image
pub async fn add(pool: &Pool<Postgres>, name: String, album: i32) {
    let _image = sqlx::query_as::<_, Image>(
        "INSERT INTO image (name, album) VALUES ($1, $2) RETURNING id, name, album"
    )
        .bind(name)
        .bind(album)
        .fetch_one(pool)
        .await
        .unwrap();
}

// Delete image
pub async fn delete(pool: &Pool<Postgres>, id: i32) {
    sqlx::query(
        "DELETE FROM image WHERE id=$1"
    )
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}
