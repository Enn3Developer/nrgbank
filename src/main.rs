use actix_web::{App, HttpServer};
use nrgbank::api::auth::{login, register};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    let address = "127.0.0.1";
    #[cfg(not(debug_assertions))]
    let address = "0.0.0.0";

    HttpServer::new(|| App::new().service(login).service(register))
        .bind((address, 5123))?
        .run()
        .await
}
