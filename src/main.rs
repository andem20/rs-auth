use actix_web::{App, HttpServer};
use auth::routes::routes;

const ADDR: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes))
        .bind((ADDR, PORT))?
        .run()
        .await
}
