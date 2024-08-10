mod Errors;
mod Models;
use serde_json::{from_str, Value};
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
use Models::{IModels::IModels, ModelConverter};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// file path to request path
    #[arg(short, long)]
    file_path: PathBuf,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if cli.file_path.is_file() {
        let f = fs::read_to_string(cli.file_path).unwrap();
        let json: Value = from_str(f.as_str()).unwrap();
        let temp_request = match ModelConverter::converter(json) {
            Ok(r) => r,
            Err(err) => panic!("{}", err),
        };

        let response = temp_request.execute().await.unwrap();
        match response {
            Models::IModels::ExecutorResponse::Option(t) => {
                println!("t = {:?}", t)
            }
        }
    } else {
        println!("File doent exists!!!!");
    }
}
