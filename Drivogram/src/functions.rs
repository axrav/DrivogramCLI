use clap::ArgMatches;
use colored::Colorize;
use indicatif;
use indicatif::{ProgressBar, ProgressStyle};
use mime_guess;
use regex::Regex;
use reqwest::{multipart, Body, StatusCode};
use serde_json::Value;
use std::cmp::min;
use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio_util::codec::{BytesCodec, FramedRead};
extern crate byte_unit;
use byte_unit::Byte;
use kdam::prelude::*;
use kdam::{Column, RichProgress};
use tabled::{Style, Table};
#[path = "types.rs"]
mod types;

// Signup
#[tokio::main]
pub async fn signup(sub_match: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Connecting to Server......".green().bold());
    let name = sub_match.get_one::<String>("name").expect("Required");
    let client = reqwest::Client::new();
    let res: types::SignupKey = client
        .post("http://drivogram.aaravarora.in/api/signup")
        .header("NAME", name)
        .send()
        .await?
        .json()
        .await?;
    let final_resp = format!("User account has been created for {} with X-API-KEY {},\nYou Have Been Logged in with the Current Key",name.red().italic().bold(),res.x_api_key.yellow().bold().on_green());
    let _file = fs::File::create("key.txt");
    fs::write("key.txt", res.x_api_key)?;
    println!("{}", final_resp.purple().bold());
    Ok(())
}

// Login check
#[tokio::main]
pub async fn login_check(sub_match: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let key = sub_match.get_one::<String>("X-API-KEY").expect("Required");
    let client = reqwest::Client::new();
    let response = client
        .post("http://drivogram.aaravarora.in/api/logincheck")
        .header("X-API-KEY", key)
        .send()
        .await?
        .status();
    let status = match response {
        StatusCode::OK => {
            let _file = fs::File::create("key.txt");
            fs::write("key.txt", key)?;
            Ok(true)
        }
        StatusCode::UNAUTHORIZED => Ok(false),
        _s => Err(println!(
            "{}",
            "Unable To Process, Please try later".red().bold()
        )),
    };
    match status {
        Ok(bool) => match bool {
            true => println!(
                "{}",
                "Logged in Successfully and your Key has been saved!"
                    .bright_blue()
                    .bold()
            ),
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
    let key = fs::read_to_string("key.txt")?;
    let client = reqwest::Client::new();
    let mut resp: types::UploadResponse = client
        .get("http://drivogram.aaravarora.in/api/uploads")
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
pub async fn download_file(sub_data: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let key = sub_data.get_one::<String>("filekey").expect("Required");
    let client = reqwest::Client::new();
    let u_key = fs::read_to_string("key.txt")?;
    let mut resp = client
        .get("http://drivogram.aaravarora.in/api/download")
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
    pb.replace(0, Column::text(&format!("[bold blue]{}", &filename)));
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
pub async fn upload_file(sub_data: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let file: &PathBuf = sub_data.get_one("PATH").expect("Requires a filepath");
    let client = reqwest::Client::new();
    let u_key = fs::read_to_string("key.txt")?;
    let md = tokio::fs::metadata(file).await.unwrap();
    let _total_size = md.len();
    let filenew: Result<&PathBuf, ()> = match md.is_dir() {
        false => Ok(file),
        true => Err(println!(
            "{}",
            "CANNOT UPLOAD A DIRECTORY,TRY WITH FILES".red().bold()
        )),
    };
    let file_open = tokio::fs::File::open(filenew.unwrap()).await.unwrap();
    let reg = Regex::new(r".*/").unwrap();
    let file_name = filenew.unwrap().to_string_lossy().to_string();
    let final_name = reg.replace_all(&file_name, "").to_string();
    let stream = FramedRead::new(file_open, BytesCodec::new());
    let mime = mime_guess::from_path(filenew.unwrap());
    let data = mime.first().unwrap();
    let _body_file = multipart::Part::stream(Body::wrap_stream(stream))
        .file_name(final_name)
        .mime_str(&data.to_string())
        .unwrap();
    let form = multipart::Form::new().part("IN_FILE", _body_file);

    // Upload Bar
    let mut up = 0;
    let pb = ProgressBar::new(md.len());
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));
    while up < md.len() {
        let new = min(up + 223211, md.len());
        up = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(3000));
    }
    let res: types::UploadedResponse = client
        .post("http://drivogram.aaravarora.in/api/upload")
        .header("X-API-KEY", u_key)
        .multipart(form)
        .send()
        .await?
        .json()
        .await?;
    pb.finish();
    let final_message = format!("{} {} {} for the User {},\nYou can check your uploaded files by using  COMMAND: drivogram uploads",filenew.unwrap().to_string_lossy().to_string().on_bright_purple(),"Has Been Uploaded Successfully To Drivogram as".bold().yellow(), res.file_key.yellow().bold(), res.user).bold().red();
    print!("{}", final_message);

    Ok(())
}

// Delete file
#[tokio::main]
pub async fn delete_file(sub_data: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let key = sub_data.get_one::<String>("filekey").expect("Required");
    let client = reqwest::Client::new();
    let u_key = fs::read_to_string("key.txt")?;
    let resp = client
        .delete("http://drivogram.aaravarora.in/api/delete")
        .header("X-API-KEY", u_key)
        .header("FILE-KEY", key)
        .send()
        .await?
        .text()
        .await?;
    let json_data: Value = serde_json::from_str(&resp)?;
    let filedata = &json_data["file"];
    if filedata.is_null() {
        println!(
            "{} {} {}",
            "The File with key".yellow(),
            key.on_red(),
            "doesnt exists,are you sure you entered right key?"
                .bold()
                .yellow()
        )
    } else {
        let final_message = format!(
            "The File {} has been deleted sucessfully! for the User {}",
            json_data["file"].to_string().on_red().bold(),
            json_data["user"].to_string().on_purple().bold()
        )
        .bold()
        .yellow();
        println!("{}", final_message);
    }
    Ok(())
}
