use std::{borrow::Cow, collections::HashMap};

use crate::Errors::ExecutorErrors::ExecutorResponseErrors;

use super::IModels::{ExecutorResponse, IModels};
use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderName},
    Client, RequestBuilder,
};
use reqwest::{Method, Request, Url};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpModel {
    url: String,
    body: String,
    headers: HashMap<Cow<'static, String>, Cow<'static, String>>,
    r_type: String,
}

#[async_trait]
impl IModels for HttpModel {
    async fn execute(&self) -> Result<ExecutorResponse, ExecutorResponseErrors> {
        let headers = self.get_headers();
        let mut client;
        match self.r_type.as_str() {
            "PUT" | "POST" | "DELETE" | "PATCH" => {
                let request = Request::new(
                    Method::from_bytes(self.r_type.as_bytes()).unwrap(),
                    Url::from(self.url.as_str().parse().unwrap()),
                );
                client = RequestBuilder::from_parts(Client::new(), request);
                client = client.body(reqwest::Body::from(self.body.clone()));
            }
            "GET" => {
                client = Client::new().get(&self.url);
            }
            _ => client = Client::new().get("https://0.0.0.0"),
        }
        println!("pokemon : {:?}", headers);
        return Ok(ExecutorResponse::Option(
            client.headers(headers).send().await.unwrap(),
        ));
    }
}

impl Default for HttpModel {
    fn default() -> Self {
        Self {
            url: Default::default(),
            body: Default::default(),
            headers: Default::default(),
            r_type: Default::default(),
        }
    }
}

impl HttpModel {
    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        for (key, value) in self.headers.iter() {
            headers.append(
                HeaderName::from_bytes(key.as_ref().as_bytes()).unwrap(),
                value.parse().unwrap(),
            );
        }
        headers
    }
}
