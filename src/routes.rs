use actix_web::web::{get, ServiceConfig, self};

use crate::handlers::jwt_handler::issue_jwt;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/jwt").route("issue_token", get().to(issue_jwt)));
}