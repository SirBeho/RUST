mod actions;

use postgres::{Client, NoTls};

fn main() -> Result<(), postgres::Error> {
    let database_url = "postgresql://benjamin:1192141@172.17.0.2:6001/clientes";
    let mut client = Client::connect(database_url, NoTls)?;

    // Crear la tabla
    actions::create::create_table(&mut client)?;

    // Operaciones en la tabla
   // actions::create::insert_user(&mut client, "benjamin@gamil.com", "admin")?;
   // actions::create::select_all(&mut client)?;

    Ok(())
}