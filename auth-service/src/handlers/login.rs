use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UsersSchema {
    username: String,
    password: String,
}

pub async fn login(
    client: web::Data<Client>,
    login_request: web::Json<LoginRequest>,
) -> impl Responder {
    let collection = client.database("auth").collection::<UsersSchema>("users");
    let user_result = collection
        .find_one(doc! { "username": &login_request.username }, None)
        .await;

    
    match user_result {
        Ok(Some(user)) => {
            let is_authenticated = bcrypt::verify(&login_request.password, &user.password);

            match is_authenticated {
                Ok(is_authenticated) => {
                    if is_authenticated {
                        HttpResponse::Ok().json(user)
                    } else {
                        HttpResponse::NotFound().body("Wrong username or password")
                    }
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string())
            }
        },
        Ok(None) => HttpResponse::NotFound().body("Wrong username or password"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }  
}
