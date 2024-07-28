use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelErrors {
    #[error("Request type not specified or found")]
    RequestTypeNotFound(String),
}
