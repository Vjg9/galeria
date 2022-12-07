use actix_web::{put, delete, post, web, Responder, HttpResponse};
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

// Delete image
#[delete("/delete/{id}")]
pub async fn delete(data: web::Data<State>, path: web::Path<i32>) -> impl Responder {
    let pool = &*data.db.lock().unwrap();
    let id = *path;

    db::image::delete(pool, id).await;

    HttpResponse::Ok() 
}

// Update image
#[put("/update/{id}")]
pub async fn update(data: web::Data<State>, path: web::Path<i32>, params: web::Json<ImageParams>) -> impl Responder {
    let pool = &*data.db.lock().unwrap();
    let id = *path;
    let name = &params.name;
    let album = params.album;

    db::image::update(pool, id, name.to_string(), album).await;

    HttpResponse::Ok()
}
