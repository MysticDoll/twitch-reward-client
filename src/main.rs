#[macro_use]
extern crate lazy_static;
extern crate env_logger;

mod client;
mod functions;
mod twitch;

use crate::client::TwitchRewardClient;
use crate::functions::Command;
use crate::functions::oscommand::OSCommand;
use crate::twitch::{get_channel_id, AuthorizationData, AuthorizationRequest};
use futures_util::{future, pin_mut, StreamExt, SinkExt};
use std::collections::HashMap;
use serde_json::Value;
use url::Url;
use tokio::runtime::Runtime;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};


#[tokio::main]
async fn main() {
    env_logger::init();

    let token = std::env::var("TWITCH_ACCESS_TOKEN").unwrap();
    let channel_id = get_channel_id(&token);
    let data = AuthorizationData::new(
        vec![format!("channel-points-channel-v1.{}", channel_id)],
        &token,
    );
    let request = AuthorizationRequest::new("LISTEN", data);
    let connect_url = Url::parse("wss://pubsub-edge.twitch.tv:443/v1/").unwrap();
    let (ws_stream, _) = connect_async(connect_url).await.unwrap();
    let (mut write, mut read) = ws_stream.split();

    write.send(Message::Text(serde_json::to_string(&request).unwrap())).await;
    while let Some(Ok(Message::Text(msg))) = read.next().await {
        let value: Value = serde_json::from_str(&msg).unwrap_or(Value::Null);
        let data = &value["data"]["message"]
            .as_str()
            .and_then(|d| serde_json::from_str(d).ok())
            .unwrap_or(Value::Null);
        if let Some("reward-redeemed") = data["type"].as_str() {
            let reward = &data["data"]["redemption"]["reward"]["title"];
            if reward.is_string() {
                let reward = reward.as_str().unwrap();
                println!("channel point reward: {}", reward);
            }
        }

    }

}


pub async fn connect() {
    let rt = Runtime::new().unwrap();
    let mut ws = ws::WebSocket::new(|out| TwitchRewardClient::new(out, commands())).unwrap();

    rt.spawn(async move {
        ws.connect(Url::parse("wss://pubsub-edge.twitch.tv:443/v1/").unwrap())
            .unwrap();
        if let Err(e) = ws.run() {
            println!("socket connect failed {}", e);
        };
    }).await;

}

fn commands() -> HashMap<String, Box<dyn Command + Send>> {
    let mut commands: HashMap<String, Box<dyn Command + Send>> = HashMap::new();
    if let Ok(file) = std::fs::File::open("./oscommand.json") {
        let reader = std::io::BufReader::new(file);
        if let Ok(osCommands) = serde_json::from_reader::<std::io::BufReader<std::fs::File>, Vec<OSCommand>>(reader) {
            for osCommand in osCommands.iter() {
                let title = &osCommand.title;

                commands.insert(title.to_owned(), Box::new(osCommand.clone()));
            };
        };
    };

    commands
}
