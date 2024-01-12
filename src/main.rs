use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main()  -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api/v1")
                    .service(register_account)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[post("/account/register")]
async fn register_account() -> impl Responder {
    HttpResponse::Ok().body("REGISTER::ACCOUNT")
}