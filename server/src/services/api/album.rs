use actix_web::{get, put, delete, post, web, Responder, HttpResponse};
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

    match db::album::get_name(pool, name.to_string()).await.unwrap() {
        Some(_) => {
            return HttpResponse::BadRequest()
        }
        None => {
            db::album::add(pool, name.to_string(), profile).await;
            let profile = db::profile::get_id(pool, profile).await.unwrap();
            let profile_name = match profile {
                Some(profile) => profile.name,
                None => "Test".to_string()
            };
            web::block(move || std::fs::create_dir(format!("./static/{}/{}", profile_name, &params.name))).await.unwrap().unwrap();
            return HttpResponse::Ok()
        }
    };
}

// Delete album
#[delete("/delete/{id}")]
pub async fn delete(data: web::Data<State>, path: web::Path<i32>) -> impl Responder {
    let pool = &*data.db.lock().unwrap();
    let id = *path;

    let album = match db::album::get_id(pool, id).await.unwrap() {
        Some(a) => a,
        None => return HttpResponse::BadRequest()
    };

    let profile = match db::profile::get_id(pool, album.profile).await.unwrap() {
        Some(p) => p,
        None => return HttpResponse::BadRequest()
    };

    let file_path = format!("static/{}/{}", profile.name, album.name);

    if std::path::Path::new(&file_path).exists() {
        web::block(|| std::fs::remove_dir_all(file_path)).await.unwrap().unwrap();
    }

    db::album::delete(pool, id).await;

    HttpResponse::Ok() 
}

// Update album 
#[put("/update/{id}")]
pub async fn update(data: web::Data<State>, path: web::Path<i32>, params: web::Json<AlbumParams>) -> impl Responder {
    let pool = &*data.db.lock().unwrap();
    let id = *path;
    let name = &params.name;
    let profile = params.profile;

    db::album::update(pool, id, name.to_string(), profile).await;

    HttpResponse::Ok()
}

// List album's images
#[get("{id}/image/list")]
pub async fn list_images(data: web::Data<State>, path: web::Path<i32>) -> impl Responder {
    let pool = &*data.db.lock().unwrap();
    let id = *path;

    let images = db::album::list_images(pool, id).await.unwrap(); 

    let json = serde_json::to_string(&images);

    json
}
