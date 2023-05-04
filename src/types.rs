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

    pub fn get_password(&self) -> String {
        self.password.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

}