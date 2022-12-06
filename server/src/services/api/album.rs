use actix_web::{post, web, Responder, HttpResponse};
use crate::db;
use crate::state::State;
use serde::{Serialize, Deserialize};

// Album params struct
#[derive(Serialize, Deserialize, Debug)]
pub struct AlbumParams {
    name: String,
    profile: i32,
}

// Add album
#[post("/add")]
pub async fn add(data: web::Data<State>, params: web::Json<AlbumParams>) -> impl Responder {
    let pool = &*data.db.lock().unwrap();
    let name = &params.name;
    let profile = params.profile;

    db::album::add(pool, name.to_string(), profile).await;

    HttpResponse::Ok() 
}
