use actix_web::{get, delete, post, web, Responder, HttpResponse};
use crate::db;
use crate::models::Profile;
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

    db::profile::add(pool, name.to_string()).await;

    HttpResponse::Ok() 
}

// Delete profile
#[delete("/delete/{id}")]
pub async fn delete(data: web::Data<State>, path: web::Path<i32>) -> impl Responder {
    let pool = &*data.db.lock().unwrap();
    let id = *path;

    db::profile::delete(pool, id).await;

    HttpResponse::Ok() 
}
