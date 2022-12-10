use actix_web::{get, put, delete, post, web, Responder, HttpResponse};
use crate::db;
use crate::state::State;
use serde::{Serialize, Deserialize};

// List profiles
#[get("/list")]
pub async fn list(data: web::Data<State>) -> impl Responder {
    let pool = &*data.db.lock().unwrap();
    let profiles = db::profile::list(pool).await.unwrap(); 

    let json = serde_json::to_string(&profiles);

    json
}

// Todo params struct
#[derive(Serialize, Deserialize, Debug)]
pub struct TodoParams {
    name: String,
}

// Add profiles
#[post("/add")]
pub async fn add(data: web::Data<State>, params: web::Json<TodoParams>) -> impl Responder {
    let pool = &*data.db.lock().unwrap();
    let name = &params.name;

    match db::profile::get_name(pool, name.to_string()).await.unwrap() {
        Some(_) => {
            return HttpResponse::BadRequest();
        }
        None => {
            db::profile::add(pool, name.to_string()).await;
            web::block(move || std::fs::create_dir(format!("./static/{}", &params.name))).await.unwrap().unwrap();
            return HttpResponse::Ok();
        }
    }
}

// Delete profile
#[delete("/delete/{id}")]
pub async fn delete(data: web::Data<State>, path: web::Path<i32>) -> impl Responder {
    let pool = &*data.db.lock().unwrap();
    let id = *path;

    let profile = match db::profile::get_id(pool, id).await.unwrap() {
        Some(p) => p,
        None => return HttpResponse::BadRequest()
    };

    let file_path = format!("static/{}", profile.name);

    if std::path::Path::new(&file_path).exists() {
        web::block(|| std::fs::remove_dir_all(file_path)).await.unwrap().unwrap();
    }

    db::profile::delete(pool, id).await;

    HttpResponse::Ok() 
}

// Update profile 
#[put("/update/{id}")]
pub async fn update(data: web::Data<State>, path: web::Path<i32>, params: web::Json<TodoParams>) -> impl Responder {
    let pool = &*data.db.lock().unwrap();
    let id = *path;
    let name = &params.name;

    db::profile::update(pool, id, name.to_string()).await;

    HttpResponse::Ok()
}

// List profile's albums
#[get("{id}/album/list")]
pub async fn list_albums(data: web::Data<State>, path: web::Path<i32>) -> impl Responder {
    let pool = &*data.db.lock().unwrap();
    let id = *path;

    let albums = db::profile::list_albums(pool, id).await.unwrap(); 

    let json = serde_json::to_string(&albums);

    json
}
