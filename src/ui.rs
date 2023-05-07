use crate::types::Secret;
use crate::database::Store;

use eframe::egui;
use magic_crypt::{new_magic_crypt,MagicCryptTrait};
use std::rc::Rc;

#[derive(Clone)]
pub struct UpdateSecret {
    pub id: u64,
    pub secret: Secret,
    pub show_window: bool,
    pub user_to_update: String,
    pub desc_to_update: String,
    pub pass_to_update: String,
    pub encoded_new_pass: String,
    pub encryption_key: String,
    response_message: String,
    data_store: Option<Rc<Store>>

}

impl Default for UpdateSecret {
    fn default() -> Self {
        Self {
            id : 0,
            secret: Secret::default(),
            show_window : false,
            user_to_update: String::new(),
            desc_to_update: String::new(),
            pass_to_update: String::new(),
            encoded_new_pass: String::new(),
            encryption_key: String::new(),
            response_message: String::new(),
            data_store:None
        }        
    }
}

impl UpdateSecret {
    pub fn new(id: u64, user_to_update:String, desc_to_update:String, pass_to_update:String, encrpytion_key:String, secret:Secret, data_store:Rc<Store>) -> UpdateSecret{
        UpdateSecret { 
            id: id,
            secret: secret,
            show_window : true,
            user_to_update: user_to_update,
            desc_to_update: desc_to_update,
            pass_to_update: pass_to_update,
            encoded_new_pass: String::new(),
            encryption_key: encrpytion_key,
            response_message: String::new(),
            data_store: Some(data_store)
        }
    }

    pub fn ui(&mut self, ui:&mut egui::Ui) {
        
        ui.label(format!("Secret ID to be updated : {}", self.secret.get_string_id()));
        let secret_desc_label = ui.label("Secret Description:");
        ui.text_edit_singleline(&mut self.desc_to_update).labelled_by(secret_desc_label.id);
        
        let secret_username_label = ui.label("Secret Username:");
        ui.text_edit_singleline(&mut self.user_to_update).labelled_by(secret_username_label.id);

        let secret_pass_label = ui.label("Secret Password:");
        ui.add(egui::TextEdit::singleline(&mut self.pass_to_update).password(true)).labelled_by(secret_pass_label.id);

        let button_name = format!("Cancel update for {}",self.secret.get_string_id());

        if ui.button(button_name).clicked() {
            self.show_window = false;
        };

        if ui.button("Submit changes").clicked() {
            let store = self.data_store.clone().unwrap();

            if self.encryption_key.len() > 0 {
                let updater_encryptor = new_magic_crypt!(self.encryption_key.clone(), 256);

                if self.pass_to_update == self.secret.get_password() {
                    self.encoded_new_pass = self.pass_to_update.clone()
                }
                else {
                    self.encoded_new_pass = updater_encryptor.encrypt_str_to_base64(self.pass_to_update.clone());
                }
                
                
                store.update_secret(
                    self.id as usize,
                    self.user_to_update.clone(),
                    self.encoded_new_pass.clone(),
                    self.desc_to_update.clone()
                ).unwrap();

                self.show_window = false;

            }

            else {
                self.response_message = format!("Unable to perform updates. Please provide a valid encryption key first!");
            }
        }

        ui.add_space(10.00);
        ui.label(self.response_message.clone());

    }

    pub fn show(&mut self, ui:&mut egui::Ui) {

        if self.show_window == true {
            let window_title = format!("Update Secret {}", self.id);
            egui::Window::new(window_title).show(ui.ctx(), |ui| self.ui(ui));
        }

    }
}

#[derive(Clone)]
pub struct DecryptSecret {
    pub id: u64,
    pub encoded_pass: String,
    pub encryption_key: String,
    pub decoded_pass: String,
    pub show_window : bool
}

impl Default for DecryptSecret {
    fn default() -> Self {
        Self {
            id:0,
            encoded_pass:String::new(),
            encryption_key:String::new(),
            decoded_pass:String::new(),
            show_window : true
        }
    }
}

impl DecryptSecret {

    pub fn new(id:u64,encoded_pass:String,encryption_key:String) -> DecryptSecret{
        DecryptSecret {
            id: id,
            encoded_pass:encoded_pass,
            encryption_key: encryption_key,
            decoded_pass: String::new(),
            show_window: true
        }
    }


    fn decrypt_secret(&mut self) {
        if self.encryption_key.len() > 0 {
            let decryptor = new_magic_crypt!(self.encryption_key.clone(), 256);

            let secret_pass = match decryptor.decrypt_base64_to_string(self.encoded_pass.clone()) {
                Ok(pass) => {
                    pass
                }
                Err(er) => {
                    format!("Can't decrypt secret. Provided Key might be invalid, error: {}", er)
                }
            };

            self.decoded_pass = secret_pass;
        }
    }

    pub fn ui(&mut self, ui:&mut egui::Ui) {
        
        if self.encryption_key.len() > 0 {

            self.decrypt_secret();
            ui.label(format!("Secret pass for: {}", self.id));
            ui.text_edit_singleline(&mut self.decoded_pass);

        }
        else { 
            ui.add(egui::Label::new("Encryption key not specified. Can't decrypt."));
        }

        if ui.button("Done").clicked() {
            self.show_window = false;
        }

    }

    pub fn show(&mut self, ui:&mut egui::Ui) {

        if self.show_window == true {
            let window_title = format!("Secret Value for {}", self.id);
            egui::Window::new(window_title).show(ui.ctx(), |ui| self.ui(ui));
        }

    }
}

#[derive(Clone)]
pub struct CreateSecret {
    pub secret_username: String,
    pub secret_password: String,
    pub secret_desc: String,
    pub encryption_key: String,
    pub encrypted_pass: String,
    pub show_window : bool,
    response_message: String,
    data_store: Option<Rc<Store>>
}

impl Default for CreateSecret {
    fn default() -> Self {
        CreateSecret {
            secret_username:String::new(),
            secret_password:String::new(),
            secret_desc:String::new(),
            encryption_key:String::new(),
            encrypted_pass:String::new(),
            show_window:false,
            response_message: String::new(),
            data_store:None
        }
    }
}

impl CreateSecret {
    pub fn new(secret_username:String, secret_password:String, secret_desc:String, encryption_key:String, data_store:Rc<Store>) -> CreateSecret {
        CreateSecret {
            secret_username:secret_username,
            secret_password:secret_password,
            secret_desc:secret_desc,
            encryption_key:encryption_key,
            encrypted_pass:String::new(),
            show_window:true,
            response_message:String::new(),
            data_store:Some(data_store)
        }
    }

    fn encrypt_secret(&mut self) {
        if self.encryption_key.len() > 0 {

            let secret_encrpytor = new_magic_crypt!(&self.encryption_key, 256);

            self.encrypted_pass = secret_encrpytor.encrypt_str_to_base64(self.secret_password.clone());
        }
    }

    pub fn ui(&mut self, ui:&mut egui::Ui) {

        let secret_username_label = ui.label("Input Secret Username: ");
            ui.text_edit_singleline(&mut self.secret_username).labelled_by(secret_username_label.id);

            let secret_pass_label = ui.label("Input Secret Password: ");
            ui.add(egui::TextEdit::singleline(&mut self.secret_password).password(true)).labelled_by(secret_pass_label.id);

            let secret_desc_label = ui.label("Input Secret Description: ");
            ui.text_edit_singleline(&mut self.secret_desc).labelled_by(secret_desc_label.id);
            
            if ui.button("Cancel").clicked() {
                self.show_window = false;
            }

            if ui.button("Submit").clicked() {

                if self.encryption_key.len() > 0 {

                    let store = self.data_store.clone().unwrap();

                    self.encrypt_secret();

                    let new_secret = Secret::new(
                        0,
                        self.secret_username.clone(),
                        self.encrypted_pass.clone(),
                        self.secret_desc.clone()
                    );
                    
                    
                    match store.insert_secret(&new_secret) {
                        Ok(()) => {
                            self.response_message = String::from("Secret Added !");
                        }
                        
                        Err(er) => {
                            self.response_message = format!("Error adding secret. Error Message : {}", er);
                        }
                    };

                }
                
                else {
                    self.response_message = String::from("Unable to create Secret, encryption key not available.");
                }

            }

            ui.add_space(10.0);
            ui.label(self.response_message.clone());

    }

    pub fn show(&mut self, ui:&mut egui::Ui) {

        if self.show_window == true {
            let window_title = format!("Create New Secret");
            egui::Window::new(window_title).show(ui.ctx(), |ui| self.ui(ui));
        }

    }

    
}