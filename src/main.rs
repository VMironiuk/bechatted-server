use actix_web::{post, web, App, HttpServer};
use serde::Deserialize;

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

#[derive(Deserialize)]
struct NewAccountInfo {
    email: String,
    password: String
}

#[post("/account/register")]
async fn register_account(new_account_info: web::Json<NewAccountInfo>) -> String {
    format!("New Account: {}, {}", new_account_info.email, new_account_info.password)
}