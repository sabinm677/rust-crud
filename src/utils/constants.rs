use std::env;

use lazy_static::lazy_static;

lazy_static!{
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
}

fn set_address() -> String {
    dotenv::dotenv().ok();
    
    env::var("ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1".to_string())
}


fn set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap()
}