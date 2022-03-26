use std::env;

pub struct Allowlist {
    pub message_id: u64,
    pub author_id: u64,
}

pub struct Database {}

impl Database {
    pub async fn connect() {
        let db_url = env::var("MYSQL_URL").expect("Mysql urllää ei löytynyt .env tiedostosta");
    }
    pub async fn save_allowlist(&self, data: Allowlist) {}

    pub async fn get_allowlist_author(&self, message_id: u64) -> u64 {
        return 936258404920475748; // test
    }
    pub async fn remove_allowlist(&self, message_id: u64) {}
}
