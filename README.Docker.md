### Building and running your application

When you're ready, start your application by running:
`docker compose up --build`.

Your application will be available at http://localhost:8000.

### Deploying your application to the cloud

First, build your image, e.g.: `docker build -t myapp .`.
If your cloud uses a different CPU architecture than your development
machine (e.g., you are on a Mac M1 and your cloud provider is amd64),
you'll want to build the image for that platform, e.g.:
`docker build --platform=linux/amd64 -t myapp .`.

Then, push it to your registry, e.g. `docker push myregistry.com/myapp`.

Consult Docker's [getting started](https://docs.docker.com/go/get-started-sharing/)
docs for more detail on building and pushing.

### References
* [Docker's Rust guide](https://docs.docker.com/language/rust/)




 // Ejecutar la conexión en un hilo separado
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error de conexión: {}", e);
        }
    });

    // Crear la tabla
    client.batch_execute(sql!(
        r#"
        CREATE TABLE IF NOT EXISTS Clientes (
            id SERIAL PRIMARY KEY,
            nombre VARCHAR(255) NOT NULL
        )
        "#
    )).await?;

    println!("Tabla creada exitosamente");

    // Insertar algunos datos
    client.execute(
        sql!("INSERT INTO Clientes (nombre) VALUES ('Ejemplo1'), ('Ejemplo2'), ('Ejemplo3')"),
        &[],
    ).await?;

    println!("Datos insertados exitosamente");

    // Consultar la tabla y mostrar los resultados
    let rows = client.query(sql!("SELECT * FROM Clientes"), &[]).await?;

    println!("Datos de la tabla:");

    for row in &rows {
        let id: i32 = row.get("id");
        let nombre: &str = row.get("nombre");
        println!("ID: {}, Nombre: {}", id, nombre);
    }

    Ok(())