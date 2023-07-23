use actix_web::web::{self, get, ServiceConfig, post};

use crate::handlers::{jwt_handler::issue_jwt, login::login};

pub fn routes(cfg: &mut ServiceConfig) {
    let jwt_scope = web::scope("/jwt")
        .service(web::resource("/issue_token").route(get().to(issue_jwt)))
        .service(web::resource("/refresh_token").route(get().to(login)));

    let auth_scope = web::scope("/auth")
        .service(web::resource("/login").route(post().to(login)));

    cfg
        .service(jwt_scope)
        .service(auth_scope);
}
