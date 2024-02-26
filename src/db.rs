use tokio_postgres::{NoTls, Client};
use tokio::sync::Mutex;
use std::sync::Arc;
use lazy_static::lazy_static;

lazy_static! {
    static ref CLIENT: Arc<Mutex<Option<Client>>> = Arc::new(Mutex::new(None));
}

#[derive(Debug, serde::Serialize)]
pub struct Usuario {
    id: i32,
    nombre: String,
    correo: String,
    telefono: String,
}

pub async fn initialize_connection() -> Result<(), tokio_postgres::Error> {
    let (new_client, connection) = tokio_postgres::connect(
        "host=postgres-container user=benjamin password=1192141 dbname=tienda_db port=6001",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error de conexión: {}", e);
        }
    });

    *CLIENT.lock().await = Some(new_client);

    Ok(())
}

pub async fn get_users() -> Result<Vec<Usuario>, tokio_postgres::Error> {

    let cli = CLIENT.lock().await;
    let client = cli.as_ref().unwrap();

    let query = "SELECT id_cliente, nombre, correo, telefono FROM cliente";
    let rows = client.query(query, &[]).await?;

    let usuarios: Vec<Usuario> = rows
        .iter()
        .map(|row| Usuario {
            id: row.get(0),
            nombre: row.get(1),
            correo: row.get(2),
            telefono: row.get(3),
        })
        .collect();

    Ok(usuarios)
}
