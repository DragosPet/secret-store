use magic_crypt::{new_magic_crypt,MagicCryptTrait};

mod database;
use database::Store;
mod types;

fn main() {
    let mc = new_magic_crypt!("test_key", 256);

    let test_encryption = mc.encrypt_str_to_base64("test");
    println!("{}", test_encryption);

    let test_decryption = mc.decrypt_base64_to_string("oycLsdYp+7go/RbEKna/2Q==").unwrap();
    println!("{}", test_decryption);

    let store = Store::connect(String::from("db/test_conn.db")).unwrap();
    match store.create_secrets_table() {
        Ok(()) => {
            println!("Table created successfully!");
            ()
        },
        Err(er) => {
            println!("Encountered error while creating secrets table : {}", er);
            println!("Error Code : {}", er.code.unwrap());
        }
    };

    let test_secret = types::Secret::new(
        0,
        String::from("test_username"),
        mc.encrypt_str_to_base64("test_password"),
        String::from("test_description")


    );

    //store.insert_secret(&test_secret).unwrap();

    //store.update_secret(2,
    //    String::from("test_username2"),
    //    String::from("test_password2"),
    //    String::from("test_description2")
    //).unwrap();

    store.delete_secret(3).unwrap();

    let secrets = match store.fetch_secrets() {
        Ok(secrets) => {
            println!("Secrets reading successful!");
            secrets
        },
        Err(er) => {
            println!("Encountered error while fetching secrets : {}", er);
            println!("Error Code : {}", er.code.unwrap());
            Vec::new()
        }
    };

    println!("Len of secrets : {}", secrets.len());



}
