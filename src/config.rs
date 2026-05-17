use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct KommentConfig {
    pub repo: String,
    pub repo_id: Option<String>,
    pub category: Option<String>,
    pub category_id: Option<String>,
    pub mapping: String,
    pub term: String,
    pub token: Option<String>,
    pub api_url: Option<String>,
}
