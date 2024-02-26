use warp::Filter;

use crate::db;

pub async fn crear_rutas() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
            
           
            let client = match db::initialize_connection().await {
                Ok(client) => client,
                Err(_) => {
                    eprintln!("Failed to initialize the database connection");
                    // Handle the error, perhaps by returning an error response
                    return warp::reply::json(&"Error interno del servidor");
                }
            };

            let ruta_ver_todos = warp::path!("clientes")
                .and(warp::post())
                .and_then(move || async {
                    let result = db::get_cliente(client.clone()).await;
                    match result {
                        Ok(clientes) => Ok::<_, warp::Rejection>(warp::reply::json(&clientes)),
                        Err(_) => Ok::<_, warp::Rejection>(warp::reply::json(&"Error interno del servidor")),
                    }
                });
/*
            let ruta_eliminar = warp::path!("clientes" / i32)
                .and(warp::delete())
                .and_then(|id: i32| async move {
                    let result = db::delete_cliente(&client, id).await;
                    match result {
                        Ok(_) => Ok::<_, warp::Rejection>(warp::reply::json(&"Usuario eliminado")),
                        Err(_) => Ok::<_, warp::Rejection>(warp::reply::json(&"Error interno del servidor")),
                    }
                });

            let ruta_agregar = warp::path!("clientes" / i32)
                .and(warp::post())
                .and(warp::body::json())
                .and_then(|id: i32, cliente: db::Cliente| async move {
                    let result;

                    if id == 0 {
                        result = db::add_cliente(&client, cliente).await;
                    } else {
                        result = db::update_cliente(&client, id, cliente).await;
                    }

                    match result {
                        Ok(_) => Ok::<_, warp::Rejection>(warp::reply::json(&"Usuario agregado")),
                        Err(_) => Ok::<_, warp::Rejection>(warp::reply::json(&"Error interno del servidor")),
                    }
                });
*/
            let ruta_inicio = warp::path::end().and(warp::fs::file("static/index.html"));
            let ruta_static = warp::path("static").and(warp::fs::dir("static"));

            ruta_ver_todos.or(ruta_static).or(ruta_inicio)
        }
       

