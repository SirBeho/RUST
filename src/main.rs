mod routes;
mod db;

#[tokio::main]
async fn main() {
    
    

    // Define tus rutas usando Warp
    let routes = routes::crear_rutas().await;

    // Inicia el servidor usando Warp
    warp::serve(routes).run(([0, 0, 0, 0], 6001)).await;
}