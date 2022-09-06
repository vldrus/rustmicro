/* MIT License */

use std::io::Result;
use std::time::Instant;
use serde::{Serialize, Deserialize};
use actix_web::{web, App, HttpServer, Responder};

#[derive(Debug)]
struct Config {
    debug: bool,
    port: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let config = web::Data::new(Config {
        debug: true,
        port: 8080,
    });

    eprintln!("INFO  Listening: {:?}", config.clone().into_inner());

    let address = format!("0.0.0.0:{}", config.port);

    let routes = move || {
        return App::new()
            .app_data(config.clone())
            .route("/", web::get().to(index_get));
    };

    return HttpServer::new(routes)
        .bind(address)?
        .run()
        .await;
}

async fn index_get(config: web::Data<Config>) -> impl Responder {
    let start = Instant::now();

    let mut users = Vec::new();

    users.push(User{username: String::from("user1")});
    users.push(User{username: String::from("user2")});

    if config.debug {
        eprintln!("DEBUG index_get - found {} users in {} ms",
            users.len(),
            start.elapsed().as_millis());
    }

    return web::Json(users);
}
