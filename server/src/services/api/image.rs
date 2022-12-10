use actix_web::{put, delete, post, web, Responder, HttpResponse};
use actix_multipart::Multipart;
use crate::db;
use crate::state::State;
use serde::{Serialize, Deserialize};
use futures_util::TryStreamExt;
use std::io::Write;

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

    let image = match db::image::get_id(pool, id).await.unwrap() {
        Some(i) => i,
        None => return HttpResponse::BadRequest()
    };
    
    let album = match db::album::get_id(pool, image.album).await.unwrap() {
        Some(a) => a,
        None => return HttpResponse::BadRequest()
    };

    let profile = match db::profile::get_id(pool, album.profile).await.unwrap() {
        Some(p) => p,
        None => return HttpResponse::BadRequest()
    };

    let file_path = format!("static/{}/{}/{}.png", profile.name, album.name, image.name);

    if std::path::Path::new(&file_path).exists() {
        web::block(|| std::fs::remove_file(file_path)).await.unwrap().unwrap();
    }

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

    let image = match db::image::get_id(pool, id).await.unwrap() {
        Some(i) => i,
        None => return HttpResponse::BadRequest()
    };
    
    let old_album = match db::album::get_id(pool, image.album).await.unwrap() {
        Some(a) => a,
        None => return HttpResponse::BadRequest()
    };

    let old_profile = match db::profile::get_id(pool, old_album.profile).await.unwrap() {
        Some(p) => p,
        None => return HttpResponse::BadRequest()
    };

    let new_album = match db::album::get_id(pool, album).await.unwrap() {
        Some(a) => a,
        None => return HttpResponse::BadRequest()
    };

    let new_profile = match db::profile::get_id(pool, new_album.profile).await.unwrap() {
        Some(p) => p,
        None => return HttpResponse::BadRequest()
    };

    let old_file_path = format!("static/{}/{}/{}.png", old_profile.name, old_album.name, image.name);
    let file_path = format!("static/{}/{}/{}.png", new_profile.name, new_album.name, &name);

    if std::path::Path::new(&old_file_path).exists() {
        web::block(move || {
                std::fs::rename(&old_file_path, file_path).unwrap();
            }
        ).await.unwrap();
    }

    db::image::update(pool, id, name.to_string(), album).await;

    HttpResponse::Ok()
}

// Add file
pub async fn add_file(mut payload: Multipart, data: web::Data<State>) -> Result<HttpResponse, actix_web::Error> {
    let pool = &*data.db.lock().unwrap();

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();

        let filename = match content_disposition.get_filename() {
            Some(name) => name,
            None => ""
        };

        let name: Vec<&str> = filename.split(".").collect();

        if name[1] != "png" {
            return Ok(HttpResponse::BadRequest().into())
        } else {
            match db::image::get_name(pool, name[0].to_string()).await.unwrap() {
                Some(image) => {
                    let image_name = image.name;
                    let album = match db::album::get_id(pool, image.album).await.unwrap() {
                        Some(a) => a,
                        None => return Ok(HttpResponse::BadRequest().into())
                    };
                    let profile = match db::profile::get_id(pool, album.profile).await.unwrap() {
                        Some(p) => p,
                        None => return Ok(HttpResponse::BadRequest().into())
                    };
                    let filepath = format!("static/{}/{}/{}.png", profile.name, album.name, image_name);
                    if std::path::Path::new(&filepath).exists() {
                        return Ok(HttpResponse::BadRequest().into())
                    } else {
                        let mut file = web::block(move || std::fs::File::create(filepath)).await.unwrap().unwrap();
                        while let Some(chunk) = field.try_next().await? {
                            file = web::block(move || file.write_all(&chunk).map(|_| file)).await.unwrap().unwrap();
                        }
                    }
                }
                None => {
                    return Ok(HttpResponse::BadRequest().into())
                }
            }
        }
    }

    Ok(HttpResponse::Ok().into())
}

// Delete file 
#[delete("file/delete/{id}")]
pub async fn delete_file(data: web::Data<State>, path: web::Path<i32>) -> impl Responder {
    let pool = &*data.db.lock().unwrap();
    let id = *path;

    let image = match db::image::get_id(pool, id).await.unwrap() {
        Some(i) => i,
        None => return HttpResponse::BadRequest()
    };
    
    let album = match db::album::get_id(pool, image.album).await.unwrap() {
        Some(a) => a,
        None => return HttpResponse::BadRequest()
    };

    let profile = match db::profile::get_id(pool, album.profile).await.unwrap() {
        Some(p) => p,
        None => return HttpResponse::BadRequest()
    };

    let file_path = format!("static/{}/{}/{}.png", profile.name, album.name, image.name);

    if std::path::Path::new(&file_path).exists() {
        web::block(|| std::fs::remove_file(file_path)).await.unwrap().unwrap();
    } else {
        return HttpResponse::BadRequest()
    }

    HttpResponse::Ok() 
}
