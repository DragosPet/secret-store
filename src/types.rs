#[derive(Clone)]
pub struct Secret {
    id: usize,
    username: String,
    password: String,
    description: String
}

impl Secret {
    pub fn new(id: usize, username:String, password:String, description:String)-> Secret{
        Secret{
            id,
            username,
            password,
            description
        }
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_string_id(&self) -> String {
        self.id.to_string()
    }

    pub fn to_string(&self) -> String {
        format!("{}-{}-{}", self.username.clone(),self.password.clone(),self.description.clone())
    }

}