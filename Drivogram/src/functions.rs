use clap::ArgMatches;
use reqwest::StatusCode;
use colored::Colorize;
use std::fs;
extern crate byte_unit;
// use byte_unit::Byte;
// use tabled::{Table, Style};
#[path = "types.rs"] mod types;


// Signup
#[tokio::main]
pub async fn signup(sub_match: &ArgMatches) -> Result<String, Box<dyn std::error::Error>>{
    println!("{}", "Connecting to Server......".green());
    let name = sub_match.get_one::<String>("name").expect("Required");
    let client = reqwest::Client::new();
    let res : types::SignupKey = client.post("http://drivogram.aaravarora.in/api/signup")
        .header("NAME",name)
        .send()
        .await?
        .json()
        .await?;
    let final_resp = format!("User account has been created for {} with X-API-KEY {},\nYou Have Been Logged in with the Current Key",name.purple().italic().bold(),res.x_api_key.yellow().bold().on_green());
    let _file = fs::File::create("key.txt");
    fs::write("key.txt", res.x_api_key)?;
    Ok(final_resp)
    
}

// login check
#[tokio::main]
pub async fn login_check(sub_match: &ArgMatches ) -> Result<Result<bool, ()>, Box<dyn std::error::Error>> {
    let key = sub_match.get_one::<String>("X-API-KEY").expect("Required");
    let client = reqwest::Client::new();
    let response = client.post("http://drivogram.aaravarora.in/api/logincheck")
    .header("X-API-KEY", key)
    .send()
    .await?
    .status();
    let status = match response{
        StatusCode::OK => { let _file = fs::File::create("key.txt");
        fs::write("key.txt", key)?;
        Ok(true)},
        StatusCode::UNAUTHORIZED => Ok(false),
        _s => Err(println!("{}", "Unable To Process, Please try later".red().bold()))
    };
    Ok(status)
    
}
#[tokio::main]
pub async fn show_data() -> Result<(), Box<dyn std::error::Error>>{
    let key = fs::read_to_string("key.txt")?;
    let client =  reqwest::Client::new();
    let resp : types::UploadResponse  = client.get("http://drivogram.aaravarora.in/api/uploads")
    .header("X-API-KEY", key)
    .send()
    .await?
    .json()
    .await?;
    for x in resp.uploads{
        println!("{:#?}", x.filesize);
    };
    Ok(())
}

    // for mut x in resp.uploads{
    //     ///x.filesize = 45;
    //     println!("{:?}",x.filesize);
        // let got = replace(&x.filesize, )
        // let byte = Byte::from_bytes(x.filesize);
        // let adjusted_byte = byte.get_appropriate_unit(false);
        // println!("{}",adjusted_byte)


    // let table = Table::new(resp.uploads).with(Style::modern()).to_string();
    // println!("{}", table.bold());
    








    
    
    
