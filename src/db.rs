use tokio_postgres::{NoTls, Client};
use tokio::sync::Mutex;
use std::sync::Arc;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::time::Duration;




lazy_static! {
    static ref CLIENT: Arc<Mutex<Option<Client>>> = Arc::new(Mutex::new(None));
}

#[derive(Debug, Deserialize, Serialize)] 
pub struct Cliente {
    id: i32,
    nombre: String,
    correo: String,
    telefono: String,
}

pub async fn initialize_connection() -> Result<(), tokio_postgres::Error> {
    match tokio_postgres::connect(
        "host=postgres-container user=benjamin password=1192141 dbname=tienda_db port=6001",
        NoTls,
    )
    .await
    {

        
        Ok((new_client, connection)) => {
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("Error de conexión: {}", e);
                }
            });

            *CLIENT.lock().await = Some(new_client);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error al conectar a la base de datos: {:?}", e);
            tokio::time::sleep(Duration::from_secs(8)).await; 
            Err(e)
        }
    }
}

pub async fn get_cliente() -> Result<Vec<Cliente>, tokio_postgres::Error> {

    let cli = CLIENT.lock().await;
    let client = cli.as_ref().unwrap();

    let query = "SELECT id_cliente, nombre, correo, telefono FROM cliente";
    let rows = client.query(query, &[]).await?;

    let clientes: Vec<Cliente> = rows
        .iter()
        .map(|row| Cliente {
            id: row.get(0),
            nombre: row.get(1),
            correo: row.get(2),
            telefono: row.get(3),
        })
        .collect();

    Ok(clientes)
}

pub async fn delete_cliente(id: i32) -> Result<(), tokio_postgres::Error> {
    let cli = CLIENT.lock().await;
    let client = cli.as_ref().unwrap();

    let query = "DELETE FROM cliente WHERE id_cliente = $1";
    client.execute(query, &[&id]).await?;

    Ok(())
}

pub async fn add_cliente(cliente: Cliente) -> Result<(), tokio_postgres::Error> {
    let cli = CLIENT.lock().await;
    let client = cli.as_ref().unwrap();

    let query = "INSERT INTO cliente (nombre, correo, telefono) VALUES ($1, $2, $3)";
    client
        .execute(query, &[&cliente.nombre, &cliente.correo, &cliente.telefono])
        .await?;

    Ok(())
}

pub async fn update_cliente(id: i32, cliente: Cliente) -> Result<(), tokio_postgres::Error> {
    let cli = CLIENT.lock().await;
    let client = cli.as_ref().unwrap();

    let query = "UPDATE cliente SET nombre = $1, correo = $2, telefono = $3 WHERE id_cliente = $4";
    client
        .execute(query, &[&cliente.nombre, &cliente.correo, &cliente.telefono, &id])
        .await?;

    Ok(())
}
