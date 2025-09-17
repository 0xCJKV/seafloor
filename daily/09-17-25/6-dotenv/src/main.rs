use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let api_key = env::var("API_KEY")
        .expect("API_KEY must be set");

    let port = env::var("PORT")
        .expect("PORT must be set");

    let debug = env::var("DEBUG")
        .expect("DEBUG must be set");

    println!("DATABASE_URL: {}", db_url);
    println!("API_KEY: {}", api_key);
    println!("PORT: {}", port);
    println!("DEBUG: {}", debug);
}