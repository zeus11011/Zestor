mod Errors;
mod Models;
use std::{
    fs::{self},
    path::Path,
};

use serde_json::{from_str, Value};
use Models::{IModels::IModels, ModelConverter};

#[tokio::main]
async fn main() {
    let f = fs::read_to_string(Path::new("./temp.json")).unwrap();
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
}
