pub struct Allowlist {
    pub message_id: String,
    pub author_id: String,
}

pub fn save_allowlist(data: Allowlist) {}

pub fn get_allowlist(message_id: String) {}

pub fn remove_allowlist(message_id: String) {}
