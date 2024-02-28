use actix_web::{web, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use tokio_postgres::{ Client, Error as PostgresError};



#[derive(Serialize, Deserialize)]
pub struct User {
    id: Option<i32>,
    name: String,
    email: String,
    phone: String,
}

//funcion index
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../static/index.html"))
}

pub async fn get_users(db: web::Data<Arc<Mutex<Client>>>) -> impl Responder {
    let client = db.lock().unwrap();
    let result = client.query("SELECT id, name, email, phone FROM users", &[]).await;
         
    match result {
        Ok(rows) => {
            let mut users = Vec::new();
            for row in rows {
                let user = User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                    phone: row.get(3),
                  
                };
                users.push(user);
            }
            HttpResponse::Ok().json(users)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}



pub async fn save_user(db: web::Data<Arc<Mutex<Client>>>, user: web::Json<User>) -> impl Responder {
    let client = db.lock().unwrap();
    
    let result = if user.id.unwrap_or_default() != 0 {
        client
            .execute(
                "UPDATE users SET name = $1, email = $2, phone = $3 WHERE id = $4",
                &[&user.name, &user.email, &user.phone, &user.id.unwrap()],
            )
            .await
    } else {
        client
            .execute(
                "INSERT INTO users (name, email, phone) VALUES ($1, $2, $3)",
                &[&user.name, &user.email, &user.phone],
            )
            .await
    };
    
    match result {
        Ok(_) => HttpResponse::Ok().body(format!("Usuario con ID {} actualizado correctamente",user.id.unwrap())),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

pub async fn delete_user(db: web::Data<Arc<Mutex<Client>>>, user_id: web::Path<i32>) -> impl Responder {
    let client = db.lock().unwrap();
    let user_id = user_id.into_inner();

    let result = client
        .execute("DELETE FROM users WHERE id = $1", &[&user_id])
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body(format!("Usuario con ID {} eliminado correctamente", user_id)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

pub async fn init(db: web::Data<Arc<Mutex<Client>>>) -> Result<(), PostgresError> {
    let client = db.lock().unwrap();
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                name VARCHAR NOT NULL,
                email VARCHAR NOT NULL,
                phone VARCHAR NOT NULL
            )",
            &[],
        )
        .await?;
    Ok(())
}
