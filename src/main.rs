mod routes;  


use routes::{get_users, create_or_update_user, delete_user};
use actix_web::{ App,  HttpServer};
use actix_files::Files;


#[derive(Debug)]
struct AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {


    HttpServer::new(|| {
        App::new()
        .service(Files::new("/", "./static").index_file("index.html"))
        .service(get_users)
        .service(create_or_update_user)
        .service(delete_user)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}


