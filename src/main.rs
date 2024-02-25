use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Configuración de la conexión a la base de datos
    let (client, connection) = tokio_postgres::connect(
        "host=postgres-container user=benjamin password=1192141 dbname=tienda_db port=6001",
        NoTls,
    )
    .await?;

    // Manejar el resultado de la conexión
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error de conexión: {}", e);
        }
    });

    
    // Ejemplo de consulta
    let rows = client.query("SELECT id_cliente, nombre FROM cliente", &[]).await?;
    
    // Procesar resultados
    for row in rows {
        let id_cliente: i32 = row.get(0);
        let nombre: &str = row.get(1);
        println!("ID: {}, Nombre: {}", id_cliente, nombre);
    }

    Ok(())
}
