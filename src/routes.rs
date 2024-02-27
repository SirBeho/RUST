
use actix_web::{get, post, delete, put, web,  HttpResponse, Result};
use tokio_postgres::{NoTls, Client};


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub correo: String,
    pub telefono: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Cliente {
    pub id: i32,
    pub name: String,
}

pub async fn coneccion_db() -> Client {

   let connection_string = "host=localhost user=benjamin password=1192141 dbname=tienda_db";

    
    //let connection_string = "postgres://benjamin:1192141@postgres:5432/tienda_db";
    let (client, connection) =
        tokio_postgres::connect(connection_string, NoTls).await.expect("Connection Error");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}

#[get("/users")]
async fn get_users() -> Result<HttpResponse> {

    println!("Obteniendo usuarios");
    // Consulta todos los usuarios desde la base de datos
    let result = coneccion_db().await.query("SELECT * FROM users", &[]).await;

    match result {
        Ok(rows) => {
            let users: Vec<User> = rows
                .iter()
                .map(|row| {
                    User {
                        id: row.get(0),
                        name: row.get(1),
                        correo: row.get(2),
                        telefono: row.get(3),
                    }
                })
                .collect();

            Ok(HttpResponse::Ok().json(users))
        }
        Err(e) => {
            eprintln!("Error retrieving users from the database: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[post("/users/{id}")]
async fn create_or_update_user(id: web::Path<i32>, user: web::Json<User>,) -> Result<HttpResponse> {

    if *id == 0 {
        return create_user(user).await;
    }

   
    let result = coneccion_db()
        .await
        .execute(
            "UPDATE users SET name = $1, correo = $2, telefono = $3 WHERE id = $4",
            &[&user.name, &user.correo, &user.telefono, &*id],
        )
        .await;


    match result {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            eprintln!("Error creating or updating user in the database: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[delete("/users/{id}")]
async fn delete_user(id: web::Path<i32>) -> Result<HttpResponse> {
    // Elimina un usuario de la base de datos
    let result = coneccion_db()
        .await
        .execute("DELETE FROM users WHERE id = $1", &[&*id])
        .await;

    match result {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            eprintln!("Error deleting user from the database: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}


async fn create_user(user: web::Json<User>) -> Result<HttpResponse> {
    
    let result = coneccion_db()
        .await
        .execute(
            "INSERT INTO users (name, correo, telefono) VALUES ($1, $2, $3)",
            &[&user.name, &user.correo, &user.telefono],
        )
        .await;

    match result {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            eprintln!("Error creating user in the database: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn home(){
    
    //crear tabla user si no existe
    let result = coneccion_db().await.query("CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, name TEXT NOT NULL, correo TEXT NOT NULL, telefono TEXT NOT NULL)", &[]).await;

    match result {
        Ok(_) => {
            println!("Tablas creada");
        }
        Err(e) => {
            eprintln!("Error creating table: {}", e);
        }
    }

    //insertar datos
    let result = coneccion_db().await.execute("INSERT INTO users (name, correo, telefono) VALUES ('benjamin', 'usuario@example.com', '123-456-7890')" , &[]).await;

        

    let result = coneccion_db().await.query("SELECT * FROM users", &[]).await;

    match result {
        Ok(rows) => {
            for row in rows.iter() {
                let id: i32 = row.get(0);
                let name: &str = row.get(1);
                let correo: &str = row.get(2);
                let telefono: &str = row.get(3);
                
              
                println!("id: {}, name: {}, correo: {}, telefono: {}", id, name, correo, telefono);
            }
        }
        Err(e) => {
            eprintln!("Error retrieving users from the database: {}", e);
        }
    }
}

