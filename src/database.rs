pub struct Allowlist {
    pub message_id: u64,
    pub author_id: u64,
}

pub fn save_allowlist(data: Allowlist) {}

pub fn get_allowlist_author(message_id: u64) -> u64 {
    return 936258404920475748; //test
}

pub fn remove_allowlist(message_id: u64) {}
