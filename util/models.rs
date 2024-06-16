use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestData {
    pub data: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct LoadRequest {
    pub tokens: std::collections::HashMap<String, String>,
}

#[derive(Serialize, Debug)]
pub struct StoreRequest {
    pub response: Vec<String>,
}
