use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GiteaRepo {
    pub name: String,
    pub clone_url: String,
    pub private: bool,
}

#[derive(Deserialize)]
pub struct TokenResponse {
    pub sha1: String,
}
