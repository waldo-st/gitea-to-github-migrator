use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateRepoRequest {
    pub name: String,
    pub private: bool,
}
