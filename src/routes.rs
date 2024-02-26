use warp::Filter;

use crate::db;
pub async fn crear_rutas() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
   
   db::initialize_connection().await.unwrap();
   
   
    let ruta_ver_todos = warp::path!("usuarios")
    .and(warp::post())
    .and_then(|| async {
        let result = db::get_users().await;
        match result {
            Ok(usuarios) => Ok::<_, warp::Rejection>(warp::reply::json(&usuarios)),
            Err(_) => Ok::<_, warp::Rejection>(warp::reply::json(&"Error interno del servidor")),
        }
    });

    let ruta_inicio = warp::path::end().and(warp::fs::file("static/index.html"));
    let ruta_static = warp::path("static").and(warp::fs::dir("static"));

    ruta_ver_todos.or(ruta_static).or(ruta_inicio)
}



/*
let route_users = warp::path!("usuarios")
     .and(warp::post())
     .and_then(|| async move {
                 let result = db::get_users().await;
         match result {
             Ok(usuarios) => {
                 // Convertir los usuarios a JSON y responder
                 let json_response = warp::reply::json(&usuarios);
                 Ok(json_response)
             }
             Err(error) => {
                 // Manejar el error y responder con un código de error
                 eprintln!("Error al obtener usuarios: {}", error);
                 let error_response = warp::reply::with_status(
                     warp::reply::json(&"Error interno del servidor"),
                     warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                 );
                 Ok(error_response)
             }
         }
     })

     */