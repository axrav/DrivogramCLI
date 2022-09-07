use clap::ArgMatches;
use serde::{Deserialize};
use colored::Colorize;
use std::fs;
#[tokio::main]

pub async fn signup(sub_match: &ArgMatches) -> Result<String, Box<dyn std::error::Error>>{
    println!("{}", "Connecting to Server......".green());
    let name = sub_match.get_one::<String>("name").expect("Required");
    let client = reqwest::Client::new();
    let res : SignupKey = client.post("http://drivogram.aaravarora.in/api/signup")
        .header("NAME",name)
        .send()
        .await?
        .json()
        .await?;
    let final_resp = format!("User account has been created for {} with X-API-KEY {},\nYou Have Been Logged in with the Current Key",name.purple().italic(),res.x_api_key.yellow().bold());
    let _file = fs::File::create("key.txt");
    fs::write("key.txt", res.x_api_key)?;
    Ok(final_resp)
    
}


#[derive(Debug, Deserialize)]
struct SignupKey{
    #[serde(rename = "X-API-KEY")]
    x_api_key: String
}
    
    
    
