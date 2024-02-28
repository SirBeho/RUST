mod controllers;
use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::{Arc, Mutex};
use tokio_postgres::{NoTls};


#[macro_use]
extern crate serde_derive;

const DB_URL: &str = env!("DATABASE_URL");


#[actix_web::main]
async fn main() -> io::Result<()> {
    let (client, connection) = tokio_postgres::connect(DB_URL, NoTls).await.map_err(|err| {
        eprintln!("Error connecting to PostgreSQL: {}", err);
        io::Error::new(io::ErrorKind::Other, "PostgreSQL Connection Error")
    })?;

    // Spawn a runtime for handling the database connection
    tokio::spawn(async {
        if let Err(e) = connection.await {
            eprintln!("Error connecting to PostgreSQL: {}", e);
        }
    });

    let db_client = Arc::new(Mutex::new(client));

    controllers::init(web::Data::new(db_client.clone())).await.unwrap();

    //.data(db_client.clone())
    // Configura el servidor Actix Web
    HttpServer::new(move || {
        App::new()
        
            .app_data(web::Data::new(db_client.clone()))
            .service(web::resource("/").to(controllers::index))
            .route("/users", web::get().to(controllers::get_users))
            .route("/users/{id}", web::post().to(controllers::save_user))
            .route("/users/{id}", web::delete().to(controllers::delete_user))
        
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
    .map_err(|err| {
        eprintln!("Error starting Actix Web server: {}", err);
        err
    })
}




