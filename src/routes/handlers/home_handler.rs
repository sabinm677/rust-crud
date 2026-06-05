use actix_web::{Responder, get, web};

use crate::utils::api_response::{self};

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello {name}!"))
}

#[get("/test")]
pub async fn test() -> impl Responder {
    api_response::ApiResponse::new(200, "test".to_string())
}

