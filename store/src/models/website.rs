use store::Store;

impl Store {
    pub fn create_website(&self) {
        println!("Create website called");
        self.conn.execute("INSERT INTO website")
    }

    pub fn get_website(&Self) -> String {
        String::from("ID_OF_WEBSITE");
    }
}