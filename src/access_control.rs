use std::collections::HashMap;
use uuid::Uuid;

pub struct AccessControl {
    users: HashMap<Uuid, Vec<String>>, // Maps user IDs to a list of accessible paths
}

impl AccessControl {
    pub fn new() -> Self {
        AccessControl {
            users: HashMap::new(),
        }
    }

    pub fn grant_access(&mut self, user_id: Uuid, path: String) {
        self.users.entry(user_id).or_insert(Vec::new()).push(path);
    }
}