use crate::functions::{Command, CommandSuccess, CommandErr};
use reqwest::{Client, Method, Request};
use serde::Deserialize;
use serde_json::Value;
use std::str::FromStr;
use tokio::task::JoinHandle;

#[derive(Clone, Debug, Deserialize)]
pub struct HttpRequest {
    pub title: String,
    url: String,
    headers: Option<Value>,
    method: String,
    data: Option<String>,
}

impl Command for HttpRequest {
    fn exec(&self) -> JoinHandle<Result<CommandSuccess, CommandErr>> {
        println!("{:?}", &self);
        let url = self.url.clone();
        let method = self.method.clone();

        let mut client = Client::new().request(Method::from_str(&method).unwrap(), &url);

        if let Some(headers) = self.headers.as_ref().unwrap_or(&Value::Null).as_object() {
            for (key, value) in headers {
                if let Value::String(s) = value {
                    client = client.header(key, s.to_owned());
                }
            }
        }

        if let Some(data) = &self.data {
            client = client.body(data.to_owned());
        }

        tokio::spawn(async move {
            if let Err(e) = client.send().await {
                Err(CommandErr::err(&format!(
                    "Error: Failed to exec request command to `{}`, with status {}",
                    url,
                    e.status().map(|status| status.to_string()).unwrap_or("unknown".to_owned())
                )))
            } else {
                Ok(CommandSuccess::nil())
            }
        })
    }
}
