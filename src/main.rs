mod routes;
mod db;

#[tokio::main]
async fn main() {
    // Llama a tu función de inicialización aquí
    if let Err(e) = db::initialize_connection().await {
        eprintln!("Error al inicializar la conexión: {}", e);
        return;
    }

    // Define tus rutas usando Warp
    let routes = routes::crear_rutas().await;

    // Inicia el servidor usando Warp
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}