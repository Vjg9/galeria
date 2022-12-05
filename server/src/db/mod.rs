use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;

// Model modules
pub mod profile;
pub mod album;
pub mod image;

// Ititialize the database
pub async fn init() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").unwrap();

   // Create connection
   let pool = PgPoolOptions::new()
       .max_connections(5)
       .connect(&url)
       .await?;

   // Create tabels if they don't exist
   sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS profile ( id SERIAL PRIMARY KEY, name TEXT);"#
    )
       .execute(&pool)
       .await?;

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS album ( id SERIAL PRIMARY KEY, name TEXT, profile INTEGER NOT NULL, FOREIGN KEY(profile) REFERENCES profile(id) ON DELETE CASCADE);"#
    )
       .execute(&pool)
       .await?;

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS image ( id SERIAL PRIMARY KEY, name TEXT, album INTEGER NOT NULL, FOREIGN KEY(album) REFERENCES album(id) ON DELETE CASCADE);"#
    )
       .execute(&pool)
       .await?;

    Ok(pool) 
}
