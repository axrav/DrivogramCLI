use serde::Deserialize;
// use tabled::Tabled;
extern crate byte_unit;
// use byte_unit::Byte;
#[derive(Debug, Deserialize)]
// #[derive(Tabled)]
pub struct UploadData{
    #[serde(rename = "file_name")]
    pub filename : String,
    pub content : String,
    #[serde(rename = "file_size")]
    pub filesize : Option<FileSize>,
    #[serde(rename = "file_key")]
    pub filekey : String

}
#[derive(Debug, Deserialize)]
pub struct SignupKey{
    #[serde(rename = "X-API-KEY")]
    pub x_api_key: String
}



#[derive(Debug, Deserialize)]
pub struct UploadResponse{
    #[serde(rename = "User")]
    pub user : String,
    #[serde(rename = "Uploads")]
    pub uploads : Vec<UploadData>

}

#[derive(Debug, Deserialize)]
pub enum FileSize {
    String(String),
    U128(u128)
}







