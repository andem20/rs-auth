use actix_web::{App, HttpServer, web};
use auth::routes::routes;

const ADDR: &str = "0.0.0.0";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection_string = "mongodb://root:rootpassword@localhost:27017";
    let mongo_client = mongodb::Client::with_uri_str(connection_string).await.expect("Could not connect to db.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mongo_client.clone()))
            .configure(routes)
    })
        .bind((ADDR, PORT))?
        .run()
        .await
}

// TODO
// [ ] Create database for users
// [ ] Populate jwt with user id
// [ ] Create database for refresh tokens
// [ ] Check refresh token if expired
