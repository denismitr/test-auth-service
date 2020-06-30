use dotenv::dotenv;
use std::env::var;

pub fn database_url() -> String {
    dotenv().ok();
    var("DATABASE_URL").expect("DATABASE_URL is set")
}

pub fn secret_key() -> String {
    dotenv().ok();
    var("SECRET_KEY").unwrap_or_else(|_| "123abc".repeat(8))
}

pub fn domain() -> String {
    dotenv().ok();
    var("DOMAIN").unwrap_or_else(|_| "localhost".to_string())
}

pub fn port() -> u16 {
    dotenv().ok();
    var("PORT").expect("PORT is set").parse::<u16>().ok().expect("PORT should be an integer")
}

pub fn domain_url() -> String {
    dotenv().ok();
    var("DOMAIN_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
}