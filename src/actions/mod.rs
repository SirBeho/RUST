
pub mod create {
    use postgres::{Client, Error};

    #[derive(Debug)]
    pub struct User {
        pub email: String,
        pub pass: String,
    }

    pub fn create_table(client: &mut Client) -> Result<(), postgres::Error> {
        // Implementación de la creación de la tabla aquí
        let query = "
            CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                email VARCHAR(20) NOT NULL,
                pass VARCHAR (20) not NULL
            )
        ";

        client.batch_execute(query)?;
        Ok(())
    }

    pub fn insert_user(client: &mut Client, email: &str, pass:&str) -> Result<(), Error> {
        client.execute(
            "INSERT INTO users (email, pass) VALUES ($1, $2)",
            &[&email, &pass],
        )?;
        Ok(())
    }

    pub fn select_all(client: &mut Client) -> Result<(), Error> {
        for row in client.query("SELECT * FROM users", &[])? {
            let user = User {
                email: row.get(0),
                pass: row.get(1),
            };
            println!("Record: {:?}", &user);
        }
        Ok(())
    }
}
