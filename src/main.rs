use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::sync::Mutex;

#[actix_web::main]
async fn main()  -> std::io::Result<()> {
    let account_repository = web::Data::new(AccountRepository {
        accounts: Mutex::new(Vec::new())
    });

    HttpServer::new(move || {
        App::new()
            .app_data(account_repository.clone())
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

#[derive(Debug)]
struct Account {
    _email: String,
    _password: String
}

struct AccountRepository {
    accounts: Mutex<Vec<Account>>
}

#[post("/account/register")]
async fn register_account(
    new_account_info: web::Json<NewAccountInfo>,
    data: web::Data<AccountRepository>
) -> impl Responder {
    let mut accounts = data.accounts.lock().unwrap();

    if accounts.iter().any(|e| e._email == new_account_info.email) {
        let message = format!("Email {} already exists", new_account_info.email);
        return HttpResponse::Conflict().body(message)
    }

    accounts.push(Account {
        _email: new_account_info.email.clone(),
        _password: new_account_info.password.clone()
    });

    let message = format!("Successfully created new account.\nAccounts Number: {}\nAccounts, {:#?}", accounts.len(), accounts);
    HttpResponse::Ok().body(message)
}