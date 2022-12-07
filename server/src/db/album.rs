use sqlx::{
    Postgres,
    Pool
};
use crate::models::Album;

// Get album by name 
pub async fn get_name(pool: &Pool<Postgres>, name: String) -> Result<Option<Album>, sqlx::Error> {
    //let row: (i32, String, i32) = sqlx::query_as(r#"SELECT * FROM album WHERE name=$1"#)
        //.bind(&name)
        //.fetch_one(pool)
        //.await?;

    match sqlx::query_as::<_, Album>(r#"SELECT * FROM album WHERE name=$1"#)
        .bind(&name)
        .fetch_one(pool)
        .await
    {
       Ok(album) => return Ok(Some(album)) ,
       Err(_) => return Ok(None)
    }

    //let album: Album = Album {
            //id: row.0,
            //name: row.1.as_str().to_string(),
            //profile: row.2,
    //};
   
    //Ok(album)
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
