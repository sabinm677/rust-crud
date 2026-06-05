use actix_web::web;

use crate::routes::handlers::{self};


pub fn config(config: &mut web::ServiceConfig) {
    config.service(handlers::home_handler::greet)
        .service(handlers::home_handler::test);
}