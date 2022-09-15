use crate::functions::helpers::credentials_dir;
use clap::ArgMatches;
use colored::Colorize;
use regex::Regex;
use reqwest::{multipart, Body, StatusCode};
use serde_json::Value;
use std::{
    cmp::min, convert::TryFrom, fs, path::PathBuf, process, thread,
    time::Duration,
};
use tokio::io::AsyncWriteExt;
use tokio_util::codec::{BytesCodec, FramedRead};
extern crate byte_unit;
use byte_unit::Byte;
use kdam::{prelude::*, Column, RichProgress};
use tabled::{Style, Table};
#[path = "helpers.rs"]
pub mod helpers;
// Signup
#[tokio::main]
pub async fn signup(
    sub_match: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Connecting to Server......".green().bold());
    let name = sub_match.get_one::<String>("name").expect("Required");
    let client = reqwest::Client::new();
    if fs::metadata(format!("{}/credentials", credentials_dir()))
        .is_ok()
    {
        println!("{}", format!("You have already signed in and your key already exists,want to signup as new user?").bold().purple().italic())
    } else {
        let res: helpers::SignupKey = client
            .get(helpers::domain("signup"))
            .header("NAME", name)
            .send()
            .await?
            .json()
            .await?;
        let final_resp = format!("User account has been created for {} with X-API-KEY {},
    \nYou Have Been Logged in with the Current Key\n\nDONT LOOSE THIS KEY,
     THIS KEY SHOULD BE KEPT SAFELY, SAVE THIS PROPERLY!!,\n\n 
     THE KEY HAS BEEN SAVED TO $HOME/.drivogram/credentials",
    name.red().italic().bold(),
    res.x_api_key.yellow().bold().on_green());
        let content = format!("X-API-KEY = \"{}\"", res.x_api_key);
        fs::write(
            format!("{}/credentials", credentials_dir()),
            content,
        )
        .unwrap();
        println!("{}", final_resp.purple().bold());
    }
    Ok(())
}

// Login check
#[tokio::main]
pub async fn login_check(
    sub_match: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let key =
        sub_match.get_one::<String>("X-API-KEY").expect("Required");
    let client = reqwest::Client::new();
    let response = client
        .get(helpers::domain("logincheck"))
        .header("X-API-KEY", key)
        .send()
        .await?;
    let status = match response.status() {
        StatusCode::OK => Ok(true),
        StatusCode::UNAUTHORIZED => Ok(false),
        _s => Err(println!(
            "{}",
            "Unable To Process, Please try later".red().bold()
        )),
    };
    match status {
        Ok(bool) => match bool {
            true => {
                if fs::metadata(format!("{}/credentials", credentials_dir()))
        .is_ok(){
                    println!("{}", "Login Check Successfull, Your credentials are saved".bold().yellow().on_black())
                }
                else{
                    let content = format!("X-API-KEY = \"{}\"", key);
                fs::write(
                    format!("{}/credentials", credentials_dir()),
                    content,
                )?;
                let json_data: Value = serde_json::from_str(&response.text().await.unwrap())?;
                let username = &json_data["user"];
                if username.is_null() {
                    println!("Logged in Successfully with {}", key.on_red().bold());
                    process::exit(0x0100);
                }
                let msg = format!(
                    "Logged in Successfully as {} and your Key has been saved!",
                    username.to_string().italic().purple().bold()
                )
                .bright_blue()
                .bold();
                println!("{}", msg)
            }
        }
            false => println!(
                "{}",
                "Unable to Login to Drivogram, Check your key and try again!"
                    .red()
                    .bold()
            ),
        },
        Err(_) => println!("{}", "An Error Occured, Try Later".red().bold()),
    }
    Ok(())
}

// List uploads
#[tokio::main]
pub async fn show_data() -> Result<(), Box<dyn std::error::Error>> {
    let key =
        helpers::read_toml().get("X-API-KEY").unwrap().to_string();
    let client = reqwest::Client::new();
    let mut resp: helpers::UploadResponse = client
        .get(helpers::domain("uploads"))
        .header("X-API-KEY", key)
        .send()
        .await?
        .json()
        .await?;
    let mut coll = resp.uploads.iter_mut();
    for data in &mut coll {
        let bytes = Byte::from_bytes(data.filesize.parse().unwrap());
        let adjusted_byte = bytes.get_appropriate_unit(false);
        data.filesize = adjusted_byte.to_string();
    }
    let table = Table::new(resp.uploads).with(Style::modern());
    println!(
        "{}\n\n{}",
        table.to_string().bold().cyan(),
        "Above is the list of your uploaded stuff, to download any of them use the filekey"
            .bold()
            .yellow()
            .on_bright_red()
    );
    Ok(())
}

// Download files
#[tokio::main]
pub async fn download_file(
    sub_data: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let key =
        sub_data.get_one::<String>("filekey").expect("Required");
    let client = reqwest::Client::new();
    let u_key =
        helpers::read_toml().get("X-API-KEY").unwrap().to_string();
    let mut resp = client
        .get(helpers::domain("download"))
        .header("X-API-KEY", u_key)
        .header("FILE-KEY", key)
        .send()
        .await?;
    let download_size = resp.content_length().unwrap();
    let b = usize::try_from(download_size).unwrap();
    let file_ = {
        resp.headers()
            .get("x-file-name")
            .and_then(|ct_len| ct_len.to_str().ok())
            .unwrap_or("0")
    };
    let filename = String::from(file_);
    let mut pb = RichProgress::new(
        tqdm!(
            total = b,
            unit_scale = true,
            unit_divisor = 1024,
            unit = "B"
        ),
        vec![
            Column::Spinner(
                "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
                    .chars()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
                80.0,
                1.0,
            ),
            Column::text("[bold blue]?"),
            Column::Bar,
            Column::Percentage(2),
            Column::text("•"),
            Column::CountTotal,
            Column::text("•"),
            Column::Rate,
            Column::text("•"),
            Column::RemainingTime,
        ],
    );
    pb.replace(
        0,
        Column::text(&format!("[bold blue]{}", &filename)),
    );
    let mut file = tokio::fs::File::create(&filename).await?;
    let mut downloaded: usize = 0;
    while let Some(item) = resp.chunk().await? {
        downloaded += item.len();
        pb.update_to(downloaded);
        file.write(&item).await?;
    }
    pb.write(
        format!(
            "Downloaded {} Successfully to your Current directory!",
            &filename
        )
        .colorize("bold yellow"),
    );
    Ok(())
}

// Upload File

#[tokio::main]
pub async fn upload_file(
    sub_data: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let file: &PathBuf =
        sub_data.get_one("PATH").expect("Requires a filepath");
    let client = reqwest::Client::new();
    let u_key =
        helpers::read_toml().get("X-API-KEY").unwrap().to_string();
    let md = tokio::fs::metadata(file).await.unwrap();
    let _total_size = md.len();
    let filenew: Result<&PathBuf, ()> = match md.is_dir() {
        false => Ok(file),
        true => Err(println!(
            "{}",
            "CANNOT UPLOAD A DIRECTORY,TRY WITH FILES".red().bold()
        )),
    };
    let file_open =
        tokio::fs::File::open(filenew.unwrap()).await.unwrap();
    let reg = Regex::new(r".*/").unwrap();
    let file_name = filenew.unwrap().to_string_lossy().to_string();
    let final_name = reg.replace_all(&file_name, "").to_string();
    let stream = FramedRead::new(file_open, BytesCodec::new());
    let mime = mime_guess::from_path(filenew.unwrap());
    let data = mime.first().unwrap();
    let _body_file =
        multipart::Part::stream(Body::wrap_stream(stream))
            .file_name(final_name)
            .mime_str(data.as_ref())
            .unwrap();
    let form = multipart::Form::new().part("IN_FILE", _body_file);

    // Upload Bar
    let mut up = 0;
    let pb = indicatif::ProgressBar::new(md.len());
    pb.set_style(indicatif::ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));
    while up < md.len() {
        let new = min(up + 223211, md.len());
        up = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(3000));
    }
    let res: helpers::UploadedResponse = client
        .post(helpers::domain("upload"))
        .header("X-API-KEY", u_key)
        .multipart(form)
        .send()
        .await?
        .json()
        .await?;
    pb.finish();
    let final_message = format!("{} {} {} for the User {},\nYou can check your uploaded files by using  COMMAND: drivogram myuploads",filenew.unwrap().to_string_lossy().to_string().bold().cyan(),"Has Been Uploaded Successfully To Drivogram as".bold().yellow(), res.file_key.yellow().bold(), res.user).bold().red();
    print!("{}", final_message);

    Ok(())
}

// Delete file
#[tokio::main]
pub async fn delete_file(
    sub_data: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let key =
        sub_data.get_one::<String>("filekey").expect("Required");
    let client = reqwest::Client::new();
    let u_key =
        helpers::read_toml().get("X-API-KEY").unwrap().to_string();
    let resp = client
        .delete(helpers::domain("delete"))
        .header("X-API-KEY", u_key)
        .header("FILE-KEY", key)
        .send()
        .await?
        .text()
        .await?;
    let json_data: Value = serde_json::from_str(&resp)?;
    let filedata = &json_data["file"];
    match filedata.is_null() {
        true => {
            println!(
                "{} {} {}",
                "The File with key".yellow(),
                key.on_red(),
                "doesnt exists,are you sure you entered right key?"
                    .bold()
                    .yellow()
            )
        }
        false => {
            let final_message = format!(
                "The File {} has been deleted sucessfully! for the User {}",
                json_data["file"].as_str().unwrap().on_red().bold(),
                json_data["user"].as_str().unwrap().on_purple().bold()
            )
            .bold()
            .yellow();
            println!("{}", final_message);
        }
    }
    Ok(())
}

// share file
#[tokio::main]
pub async fn share_file(
    sub_data: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let key =
        sub_data.get_one::<String>("filekey").expect("Required");
    let time = sub_data.get_one::<f64>("time").expect("Required");
    let u_key =
        helpers::read_toml().get("X-API-KEY").unwrap().to_string();
    let client = reqwest::Client::new();
    let post = helpers::SharePost {
        userkey: String::from(&u_key),
        filekey: key.to_string(),
        exp: *time,
    };
    let resp = client
        .post(helpers::domain("share"))
        .json(&post)
        .header("X-API-KEY", String::from(&u_key))
        .send()
        .await?
        .text()
        .await?;
    let json_data: Value = serde_json::from_str(&resp)?;
    let link = &json_data["link"];
    match link.is_null() {
        true => {
            println!(
                "{} {} {}",
                "The File with key".yellow(),
                key.on_red(),
                "doesnt exists,are you sure you entered right key?"
                    .bold()
                    .yellow()
            );
        }
        false => {
            let final_message = format!(
            "The File sharing has been enabled sucessfully for {} hours! \n\nHere is the LINK:: {}",
            time.to_string().bold().on_red(), json_data["link"].as_str().unwrap().red().bold().italic());
            println!("{}", final_message.bold().yellow());
        }
    }
    Ok(())
}
