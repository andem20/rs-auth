use actix_web::{HttpResponse, Responder};
use jsonwebtoken::{Header, EncodingKey};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    name: String
}

pub async fn issue_jwt() -> impl Responder {
    let claims = Claims {
        name: "Anders".to_string()
    };

    let jwt = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(b"secret"));
    HttpResponse::Ok().body(jwt.unwrap())
}
