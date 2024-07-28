use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutorResponseErrors {
    #[error("Request type not specified or found")]
    RequestTypeNotFound(String),
}
