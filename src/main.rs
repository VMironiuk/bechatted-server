use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main()  -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api/v1")
                    .service(hello)
                    .service(register_account)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Bonjour!")
}

#[post("/account/register")]
async fn register_account() -> impl Responder {
    HttpResponse::Ok().body("REGISTER::ACCOUNT")
}