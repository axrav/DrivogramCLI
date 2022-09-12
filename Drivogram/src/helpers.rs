use serde::{Deserialize, Serialize};
use std::env::var;
use tabled::Tabled;
#[derive(Debug, Deserialize, Tabled)]
pub struct UploadData {
    #[serde(rename = "file_name")]
    pub filename: String,
    pub content: String,
    #[serde(rename = "file_size")]
    pub filesize: String,
    #[serde(rename = "file_key")]
    pub filekey: String,
}
#[derive(Debug, Deserialize)]
pub struct SignupKey {
    #[serde(rename = "X-API-KEY")]
    pub x_api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct UploadResponse {
    #[serde(rename = "User")]
    pub user: String,
    #[serde(rename = "Uploads")]
    pub uploads: Vec<UploadData>,
}

#[derive(Debug, Deserialize)]
pub struct UploadedResponse {
    pub msg: String,
    pub file_key: String,
    pub user: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteResponse {
    pub message: String,
    pub file: String,
    pub user: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharePost {
    pub userkey: String,
    pub filekey: String,
    pub exp: f64,
}

pub fn domain(path: &str) -> String {
    let domain_name: String = match var("DOMAIN_NAME") {
        Ok(domain) => format!("{}{}{}", domain, "/api/", path),
        Err(_) => format!(
            "{}{}",
            "http://drivogram.aaravarora.in/api/", path
        ),
    };
    domain_name
}
