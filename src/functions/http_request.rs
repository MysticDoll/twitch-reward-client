use crate::client::TwitchRewardClient;
use crate::functions::Command;
use reqwest::{Client, Method, Request};
use serde::Deserialize;
use serde_json::Value;
use std::str::FromStr;
use ws::Result;

#[derive(Clone, Debug, Deserialize)]
pub struct HttpRequest {
    pub title: String,
    url: String,
    headers: Option<Value>,
    method: String,
    data: Option<String>,
}

impl Command for HttpRequest {
    fn exec(&self, _client: &TwitchRewardClient) -> Result<()> {
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

        tokio::spawn(async {
            client.send().await
        });
        tokio::spawn(async {
            println!("workaround");
        });


        Ok(())
    }
}
