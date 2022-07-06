use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ChannelInfo {
    user_id: String,
}

pub(crate) fn get_channel_id(token: &str) -> String {
    Client::new()
        .get("https://id.twitch.tv/oauth2/validate")
        .header("Authorization", format!("OAuth {}", token))
        .send()
        .map_err(|e| panic!("error in get_channel_id {}", e.to_string()))
        .and_then(|r| r.json())
        .map(|r: ChannelInfo| r.user_id)
        .unwrap()
}

#[derive(Serialize)]
pub(crate) struct AuthorizationData {
    topics: Vec<String>,
    auth_token: String,
}

#[derive(Serialize)]
pub(crate) struct AuthorizationRequest {
    r#type: String,
    data: AuthorizationData,
}

impl AuthorizationRequest {
    pub fn new<S: Into<String>>(r#type: S, data: AuthorizationData) -> Self {
        Self {
            r#type: r#type.into(),
            data,
        }
    }
}

impl AuthorizationData {
    pub fn new<S: Into<String>>(topics: Vec<String>, auth_token: S) -> Self {
        Self {
            topics,
            auth_token: auth_token.into(),
        }
    }
}
