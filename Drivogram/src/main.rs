mod args;
mod functions;
use args::arguments;
use colored::Colorize;
use functions::helpers::credentials_dir;
use std::fs;
fn main() {
    let _file_dir = fs::create_dir_all(credentials_dir());
    let data = arguments().get_matches();
    if let Some((name, sub_match)) = data.subcommand() {
        match name {
            "source" => {
                println!(
                    "{} {}",
                    "Here is the source of Drivogram:"
                        .yellow()
                        .bold(),
                    "https://github.com/Axrav/Drivogram"
                        .cyan()
                        .bold()
                );
            }
            "signup" => {
                functions::signup(sub_match).unwrap();
            }
            "login" => {
                functions::login_check(sub_match).unwrap();
            }
            "myuploads" => {
                functions::show_data().unwrap();
            }
            "download" => {
                functions::download_file(sub_match).unwrap();
            }
            "upload" => {
                functions::upload_file(sub_match).unwrap();
            }
            "delete" => {
                functions::delete_file(sub_match).unwrap();
            }
            "share" => {
                functions::share_file(sub_match).unwrap();
            }
            _ => {
                println!("{}","NOT A VALID COMMAND,TRY WITH A VALID COMMAND, CHECKOUT --help section".yellow().bold())
            }
        }
    }
}
