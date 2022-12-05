use std::sync::Mutex;
use sqlx::{Pool, Postgres};

pub struct State {
    pub db: Mutex<Pool<Postgres>>,
}
