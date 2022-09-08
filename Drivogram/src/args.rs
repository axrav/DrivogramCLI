use clap::{arg, Arg, Command};
use std::path::PathBuf;
pub fn cli<'a>() -> Command<'a> {
    Command::new("drivogram")
        .author("Aarav Arora <aaravarora.works@gmail.com>")
        // Interactive mode to be worked on later
        //.arg(arg!(-i --interactive "Enter the Interactive Mode"))
        .about("a CLI tool to interact with Drivogram(A drive within yourself)")
        .subcommand_required(true)
        .version("1.0.2")
        .allow_external_subcommands(true)
        .after_help("drivogram is an open sourced cloud drive based on telegram, Pass a subcommand to proceed, for more info checkout https://github.com/Axrav/Drivogram")
        .subcommand(
            Command::new("source")
                .about("Get the source code of drivogram"),

        )

        .subcommand(
            Command::new("login")
                .about("Login to your own drive,Drivogram, Login with X-API-KEY")
                .arg(Arg::new("X-API-KEY"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("signup")
                .arg_required_else_help(true)
                .arg(arg!(-n --name <NAME> "Enter your name"))
                .help("ERROR: Signup to Drivogram with your name, Use -n or --name to pass your name"),

        )
        .subcommand(
            Command::new("upload")
                .about("uploads the file to your drive")
                .arg_required_else_help(true)
                .help("upload file to drivogram, max size should be 1800MB,Please pass the file path")
                .arg(arg!(<PATH> ... "Things to upload").value_parser(clap::value_parser!(PathBuf))),

        )
        .subcommand(
            Command::new("download")
                .about("downloads the file to your local disk")
                .arg_required_else_help(true)
                .help("DOWNLOAD File from Drivogram,Please pass the filekey to Download with -f or --filekey")
                .arg(arg!(-f --filekey <FILEKEY> ... "File to download"))


        )
        .subcommand(
            Command::new("myuploads")
                .help("Get the list of all your uploads"),

        )
}
