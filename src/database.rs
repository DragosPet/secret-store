use std::usize;

use crate::types::Secret;
use sqlite::{Connection, Error};

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
            let description = record.read::<&str,_>("description");

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

    pub fn insert_secret(&self, secret:&Secret) -> Result<(), Error> {
        let query = "INSERT INTO passwords (username, password, description) VALUES(?,?,?)";
        let mut insert_statement = self.database_connection.prepare(query).unwrap();
        insert_statement.bind((1, secret.get_username().as_str())).unwrap();
        insert_statement.bind((2, secret.get_password().as_str())).unwrap();
        insert_statement.bind((3, secret.get_description().as_str())).unwrap();

        insert_statement.next()?;

        Ok(())

    }

    pub fn update_secret(&self, id:usize, username:String, password:String, description:String) -> Result<(), Error> {
        let query = "UPDATE passwords
        SET 
            username = ?,
            password = ?,
            description = ?
        WHERE
            id =?
        ";

        let mut update_statement = self.database_connection.prepare(query).unwrap();
        update_statement.bind((1, username.as_str())).unwrap();
        update_statement.bind((2, password.as_str())).unwrap();
        update_statement.bind((3, description.as_str())).unwrap();
        update_statement.bind((4, id as f64)).unwrap();

        update_statement.next()?;

        Ok(())

    }

    pub fn delete_secret(&self, id:usize) -> Result<(), Error> {
        let query = "DELETE FROM passwords WHERE id=?";
        let mut delete_statement = self.database_connection.prepare(query).unwrap();
        delete_statement.bind((1, id as f64)).unwrap();
        delete_statement.next()?;
        Ok(())
        
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    pub const TEST_DB_PATH:&str = "test_files/test_db.db";

    #[test]
    fn test_connection() {
        
        let store = Store::connect(String::from(TEST_DB_PATH)).unwrap();
        assert_eq!(1,1);
    }

    #[test]
    fn test_insert_secret() {
        let store = Store::connect(String::from(TEST_DB_PATH)).unwrap();

        store.create_secrets_table().unwrap();

        let test_secret = Secret::new(
            0,
            String::from("test_username"),
            String::from("test_password"),
            String::from("test_description")
        );

        store.insert_secret(&test_secret).unwrap();

        assert_eq!(1,1);
    }

    #[test]
    fn test_update_secret() {
        let store = Store::connect(String::from(TEST_DB_PATH)).unwrap();

        store.update_secret(1,
            String::from("test_username_2"),
            String::from("test_password2"),
            String::from("Test Description 2")
        ).unwrap();

        assert_eq!(1,1);
    }

    #[test]
    fn test_all_secrets_available() {
        let store = Store::connect(String::from(TEST_DB_PATH)).unwrap();

        let secrets = store.fetch_secrets().unwrap();

        assert!(secrets.len() > 0);
    }

    #[test]
    fn test_delete_secret() { 
        let store = Store::connect(String::from(TEST_DB_PATH)).unwrap();

        store.delete_secret(1).unwrap();
        assert_eq!(1,1);
    }

}