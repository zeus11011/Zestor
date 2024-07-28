use serde_json::Value;

use crate::Models::HttpModels;
use crate::Models::IModels;

use crate::Errors::ModelErrors;

pub fn converter(request: Value) -> Result<Box<dyn IModels::IModels>, ModelErrors::ModelErrors> {
    match &request["type"] {
        Value::String(r_type) => match r_type.as_str() {
            "http" => {
                let temp: HttpModels::HttpModel =
                    serde_json::from_value(request["data"].clone()).unwrap();
                return Ok(Box::new(temp));
            }
            _ => Err(ModelErrors::ModelErrors::RequestTypeNotFound(
                "asas".to_string(),
            )),
        },
        _ => {
            return Err(ModelErrors::ModelErrors::RequestTypeNotFound(
                "".to_string(),
            ))
        }
    }
}
