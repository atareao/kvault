use actix_web::{post, get, delete, web, HttpResponse, HttpRequest,
                http::header::ContentType, Responder};
use log::info;
use serde_json::json;
use sqlx::SqlitePool;
use crate::models::user::{User, Role, NewUser};

#[post("/v1/user")]
pub async fn create(req: HttpRequest, pool: web::Data<SqlitePool>, new: web::Json<NewUser>) -> impl Responder{
    match User::from_request(&req, &pool).await {
        Some(user) =>  if user.is_admin(){
                let role = Role::User.to_string();
                match User::create(&pool, &role, &new.into_inner()).await {
                    Ok(new_user) => HttpResponse::Created()
                        .content_type(ContentType::json())
                        .body(json!({"token": new_user.token}).to_string()),
                    Err(_) => HttpResponse::UnprocessableEntity().finish(),
                }
            }else{
                HttpResponse::Unauthorized().finish()
            },
        None => HttpResponse::Unauthorized().finish(),
    }
}

#[get("/v1/user/{username}")]
pub async fn read_one(req: HttpRequest, pool: web::Data<SqlitePool>, path: web::Path<String>) -> impl Responder{
    let username = path.into_inner();
    info!("username: '{}'", &username);
    match User::from_request(&req, &pool).await {
        Some(user) =>  if user.is_admin(){
            if username == "" {
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(serde_json::to_string(&user).unwrap())
            }else{
                match User::search(&pool, &username).await {
                    Ok(searched_user) =>  HttpResponse::Ok()
                            .content_type(ContentType::json())
                            .body(serde_json::to_string(&searched_user).unwrap()),
                    Err(_) => HttpResponse::NotFound().finish(),
                }
            }
        }else{
            HttpResponse::Unauthorized().finish()
        },
        None => HttpResponse::Unauthorized().finish(),
    }
}

#[get("/v1/user")]
pub async fn read(req: HttpRequest, pool: web::Data<SqlitePool>) -> impl Responder{
    match User::from_request(&req, &pool).await {
        Some(user) =>  HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&user).unwrap()),
        None => HttpResponse::UnprocessableEntity().finish(),
    }
}

#[delete("/v1/user")]
pub async fn delete(req: HttpRequest, pool: web::Data<SqlitePool>, username: String) -> impl Responder{
    match User::from_request(&req, &pool).await {
        Some(user) =>  if user.is_admin(){
                match User::delete(&pool, &username).await {
                    Ok(_) => HttpResponse::Ok().finish(),
                    Err(_) => HttpResponse::NotFound().finish(),
                }
            }else{
                HttpResponse::Unauthorized().finish()
            },
        None => HttpResponse::Unauthorized().finish(),
    }
}
