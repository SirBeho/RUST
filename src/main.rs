use tokio_postgres::{NoTls, Error,Client};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Configuración de la conexión a la base de datos
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=benjamin password=1192141 dbname=tienda_db port=5432",
        NoTls,
    )
    .await?;

    // Manejar el resultado de la conexión
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error de conexión: {}", e);
        }
    });

    if let Err(e) = crear_tabla(&client).await {
        eprintln!("Error al llamar datos: {}", e);
    }

    if let Err(e) = llamar_datos(&client).await {
        eprintln!("Error al llamar datos: {}", e);
    }

    
    Ok(())
}

//crear tabla e insertar datos por defecto
async fn crear_tabla(client: &Client) -> Result<(), Error> {
  

    client
        .execute(
            "CREATE TABLE IF NOT EXISTS cliente (
                id_cliente SERIAL PRIMARY KEY,
                nombre VARCHAR(50) NOT NULL,
                correo VARCHAR(50) NOT NULL,
                telefono VARCHAR(50) NOT NULL
            )",
            &[],
        )
        .await?;

    client
        .execute(
            "INSERT INTO cliente (nombre, correo, telefono) values ('Benjamin', 'jose', '12345678')",
            &[],
        )
        .await?;
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
