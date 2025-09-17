use actix_web::{web, App, HttpServer, Result, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// GET endpoint that returns JSON
async fn get_user() -> Result<HttpResponse> {
    let user = User {
        id: 1,
        name: "0xCJKV".to_string(),
        email: "anon@example.com".to_string(),
    };

    Ok(HttpResponse::Ok().json(user))
}

// POST endpoint that accepts JSON
async fn create_user(user: web::Json<User>) -> Result<HttpResponse> {
    println!("Recieved user: {} with email {}", user.name, user.email);

    Ok(HttpResponse::Created().json(&*user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Data API server listening on 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/user", web::get().to(get_user))
            .route("/user", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
