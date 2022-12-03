use actix_web::{get, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello World!"
}
