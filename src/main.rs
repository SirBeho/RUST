#[macro_use]
extern crate rocket;
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;


use diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket::response::content;

// Definir la estructura de la tabla
table! {
    Students (id) {
        id -> Integer,
        name -> Text,
        age -> Integer,
    }
}

// Modelo de datos para la tabla 'students'
#[derive(Queryable)]
struct Student {
    id: i32,
    name: String,
    age: i32,
}

// Establecer la conexión a la base de datos PostgreSQL
fn establish_connection() -> PgConnection {
    let database_url = "postgresql://benjamin:1192141@172.17.0.2/clientes";
    PgConnection::establish(&database_url).expect("Error connecting to the database")
}

#[get("/data")]
fn get_data() -> Result<String, &'static str> {
    let connection = establish_connection();
    let students: Vec<Student> = Students::table.load(&connection);
    match students {
        Ok(students) => {
            for student in students {
                // Considera devolver una cadena formateada en la respuesta HTTP o registrar los detalles en un registro
                println!("ID: {}, Name: {}, Age: {}", student.id, student.name, student.age);
            }
            Ok("Data endpoint!".to_string())
        }
        Err(_) => Err("Error loading students"),
    }
}

#[get("/")]
fn index() -> &'static str {
    embed_migrations!();
    "Hello, Docker!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_data])
}

use diesel_migrations::embed_migrations;
run_migrations(&connection);
