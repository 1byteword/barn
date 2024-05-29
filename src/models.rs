use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestData {
    pub data: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct TokenizedResponse {
    pub tokens: std::collections::HashMap<String, String>,
}

#[derive(Serialize, Debug)]
pub struct DetokenizedResponse {
    pub response: Vec<String>,
}
