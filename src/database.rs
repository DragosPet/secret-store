use std::usize;

use crate::types::Secret;
use sqlite::{Connection, Error, Value};

pub struct Store {
    database_connection : Connection 
}

impl Store {
    pub fn connect(connection_path: String) -> Result<Store,Error>{
        let conn = sqlite::open(connection_path).unwrap();
        let store = Store{
            database_connection: conn
        };
        Ok(store)
    }

    pub fn create_secrets_table(&self) -> Result<(), Error> {
        let statement = "CREATE TABLE IF NOT EXISTS passwords(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username    TEXT NOT NULL,
            password    TEXT NOT NULL,
            description TEXT NOT NULL
        )";
        self.database_connection.execute(statement)?;
        Ok(())
    }

    pub fn fetch_secrets(&self) -> Result<Vec<Secret>, Error> {
        let select_statement = "SELECT * FROM passwords";
        let prepared_statement = self.database_connection.prepare(select_statement).unwrap();
        let all_secrets= prepared_statement.into_iter();

        let mut secrets_list:Vec<Secret> = Vec::new();

        for record in all_secrets.map(|row| row.unwrap()) {
            let id = record.read::<i64, _>("id");
            let username = record.read::<&str,_>("username");
            let password = record.read::<&str,_>("password");
            let description = record.read::<&str,_>("password");

            let secret = Secret::new(
                id as usize,
                String::from(username),
                String::from(password),
                String::from(description)
            );

            secrets_list.push(secret);
        };

        Ok(secrets_list)

    }

    pub fn insert_secret(&self, secret:Secret) -> Result<(), Error> {
        let query = "INSERT INTO passwords (username, password, description) VALUES(?,?,?)";
        let mut insert_statement = self.database_connection.prepare(query).unwrap();
        insert_statement.bind((1, secret.get_username().as_str())).unwrap();
        insert_statement.bind((2, secret.get_password().as_str())).unwrap();
        insert_statement.bind((3, secret.get_description().as_str())).unwrap();

        insert_statement.next().unwrap();
        
        Ok(())

    }
}