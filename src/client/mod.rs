use crate::functions::Command;
use crate::twitch::{get_channel_id, AuthorizationData, AuthorizationRequest};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use ws::{Handler, Handshake, Message, Result as WSResult, Sender};

pub struct TwitchRewardClient {
    out: Sender,
    commands: HashMap<String, Box<dyn Command + Send>>,
}

impl Handler for TwitchRewardClient {
    fn on_open(&mut self, _: Handshake) -> WSResult<()> {
        let token = std::env::var("TWITCH_ACCESS_TOKEN").map_err(|e| {
            println!("error occured with access token");
            let kind = ws::ErrorKind::Custom(Box::new(e));
            ws::Error::new(kind, "Could not get oauth token")
        })?;
        let channel_id = get_channel_id(&token);
        let data = AuthorizationData::new(
            vec![format!("channel-points-channel-v1.{}", channel_id)],
            &token,
        );
        let request = AuthorizationRequest::new("LISTEN", data);
        let r = self.out.send(serde_json::to_string(&request).unwrap());

        self.ping();

        r
    }

    fn on_message(&mut self, msg: Message) -> WSResult<()> {
        println!("onmessage: {:?}", msg);

        let value: Value = serde_json::from_str(&msg.into_text()?).unwrap_or(Value::Null);

        if let Some("MESSAGE") = value["type"].as_str() {
            let data = &value["data"]["message"]
                .as_str()
                .and_then(|d| serde_json::from_str(d).ok())
                .unwrap_or(Value::Null);
            if let Some("reward-redeemed") = data["type"].as_str() {
                let reward = &data["data"]["redemption"]["reward"]["title"];
                if reward.is_string() {
                    let reward = reward.as_str().unwrap();
                    self.handle_reward(reward)
                        .and_then(|c| c.exec(&self).ok())
                        .map(|_| println!("success execution {}", reward));
                }
            }
        }
        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        let c: u16 = code.into();
        println!("socket closed code: {}, reason: {}", c, reason);
        crate::connect();
        println!("trying reconnect");
    }
}

#[derive(Debug)]
enum ParseError {
    InvalidSource(String),
    CaptureFailed,
    RegexError,
    InvalidMessageFormat,
    CommandNotFound,
}

impl TwitchRewardClient {
    pub fn new(out: Sender, commands: HashMap<String, Box<dyn Command + Send>>) -> Self {
        Self { out, commands }
    }

    pub fn handle_reward(&self, title: &str) -> Option<&Box<dyn Command + Send>> {
        println!("raw reward: {}", title);
        self.commands.get(title)
    }

    pub fn ping(&self) {
        let out = self.out.clone();
        std::thread::spawn(move || loop {
            out.send("{\"type\": \"PING\"}");
            std::thread::sleep_ms(14000);
        });
    }
}
