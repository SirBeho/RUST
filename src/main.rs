use tokio_postgres::{NoTls, Error,Client};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Configuración de la conexión a la base de datos
    let (client, connection) = tokio_postgres::connect(
        "host=postgres-container user=benjamin password=1192141 dbname=tienda_db port=5432",
        NoTls,
    )
    .await?;

    // Manejar el resultado de la conexión
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error de conexión: {}", e);
        }
    });

    if let Err(e) = llamar_datos(&client).await {
        eprintln!("Error al llamar datos: {}", e);
    }

    
    Ok(())
}

async fn llamar_datos(client: &Client) -> Result<(), Error> {

    let rows = client.query("SELECT id_cliente,nombre, correo , telefono FROM cliente", &[]).await?;

    for row in rows {
        let id_cliente: i32 = row.get(0);
        let nombre: &str = row.get(1);
        let correo: &str = row.get(2);
        let telefono: &str = row.get(3);
        println!("ID: {}, Nombre: {}, Correo: {}, Telefono: {}", id_cliente, nombre, correo, telefono);
    }
       Ok(())
}
