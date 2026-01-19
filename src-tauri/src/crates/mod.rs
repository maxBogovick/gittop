pub mod client;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crate {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub downloads: i64,
    pub max_version: String,
    pub newest_version: Option<String>,
    pub updated_at: String,
    pub created_at: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CratesResponse {
    pub crates: Vec<Crate>,
}
