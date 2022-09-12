mod args;
mod functions;
use args::cli;
use colored::Colorize;
use dotenv::dotenv;
fn main() {
    dotenv().ok();
    let data = cli().get_matches();
    match data.subcommand() {
        Some(("source", _sub_matches)) => {
            println!(
                "{} {}",
                "Here is the source of Drivogram:".yellow().bold(),
                "https://github.com/Axrav/Drivogram".cyan().bold()
            );
        }
        Some(("signup", sub_match)) => {
            functions::signup(sub_match).unwrap();
        }
        Some(("login", sub_match)) => {
            functions::login_check(sub_match).unwrap();
        }
        Some(("myuploads", _)) => {
            functions::show_data().unwrap();
        }

        Some(("download", sub_data)) => {
            functions::download_file(sub_data).unwrap();
        }
        Some(("upload", sub_data)) => {
            functions::upload_file(sub_data).unwrap();
        }
        Some(("delete", sub_data)) => {
            functions::delete_file(sub_data).unwrap();
        }
        Some(("share", sub_data)) => {
            functions::share_file(sub_data).unwrap();
        }
        _ => println!(
            "{}",
            "NOT A VALID COMMAND,TRY WITH A VALID COMMAND, CHECKOUT --help section"
                .yellow()
                .bold()
        ),
    }
}
