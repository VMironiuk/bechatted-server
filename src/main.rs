use actix_web::{get, HttpResponse, Responder};

#[actix_web::main]
async fn main() {
    println!("Hello, world!");
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Bonjour!")
}