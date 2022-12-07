use actix_web::{post, web, Responder, HttpResponse};
use crate::db;
use crate::state::State;
use serde::{Serialize, Deserialize};

// Album params struct
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageParams {
    name: String,
    album: i32,
}

// Add image
#[post("/add")]
pub async fn add(data: web::Data<State>, params: web::Json<ImageParams>) -> impl Responder {
    let pool = &*data.db.lock().unwrap();
    let name = &params.name;
    let album = params.album;

    match db::image::get_name(pool, name.to_string()).await.unwrap() {
        Some(_) => {
            return HttpResponse::BadRequest()
        }
        None => {
            db::image::add(pool, name.to_string(), album).await;
            return HttpResponse::Ok()
        }
    };
}
