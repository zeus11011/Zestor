use crate::Errors::ExecutorErrors::ExecutorResponseErrors;
use async_trait::async_trait;
use reqwest;

#[async_trait]
pub trait IModels {
    async fn execute(&self) -> Result<ExecutorResponse, ExecutorResponseErrors>;
}

pub enum ExecutorResponse {
    Option(reqwest::Response),
}
