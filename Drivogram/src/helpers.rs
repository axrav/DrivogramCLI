use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::var;
use std::fs;
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

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
pub fn get_domain(route: &str) -> String {
    let domain = format!("{}/domain.toml", credentials_dir());
    let validity = path_exists(&domain);
    if !validity {
        format!("http://drivogram.aaravarora.in/api/{}", route)
    } else {
        let strd = fs::read_to_string(domain).unwrap();
        let domain_name: HashMap<String, String> =
            toml::from_str(&strd).unwrap();
        format!(
            "{}/api/{}",
            domain_name.get("DOMAIN-NAME").unwrap().to_string(),
            route
        )
    }
}
pub fn credentials_dir() -> String {
    let path = var("HOME").unwrap();
    format!("{}/.drivogram", path)
}

pub fn read_toml() -> HashMap<String, String> {
    let cred = format!("{}/credentials", credentials_dir());
    let strd =
        (fs::read_to_string(&cred)).unwrap_or("None".to_string());
    let data: HashMap<String, String> = match toml::from_str(&strd) {
        Ok(data) => data,
        Err(_) => HashMap::new(),
    };
    data
}
