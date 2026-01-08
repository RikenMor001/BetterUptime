use store::Store;

impl Store {
    pub fn create_user(&self) {
        println!("Create user called");
        self.conn.execute("INSERT INTO website")
    }

    pub fn get_user(&Self) -> String {
        String::from("ID_OF_USER");
    }
}