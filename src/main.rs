#[macro_use]
extern crate lazy_static;
extern crate env_logger;

// mod client;
mod websocket;
mod functions;
mod twitch;

// use crate::client::TwitchRewardClient;
use crate::functions::Command;
use crate::functions::oscommand::OSCommand;
use crate::twitch::{get_channel_id, AuthorizationData, AuthorizationRequest};
use crate::websocket::{connect_websocket, handle_websocket};
use std::collections::HashMap;
use url::Url;
use tokio::runtime::Runtime;


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
    let (ws_stream, _) = connect_websocket("wss://pubsub-edge.twitch.tv:443/v1/").await.expect("Failed to establish websocket connection.");

    handle_websocket(request, ws_stream, commands()).await;
}


//pub async fn connect() {
//    let rt = Runtime::new().unwrap();
//    let mut ws = ws::WebSocket::new(|out| TwitchRewardClient::new(out, commands())).unwrap();
//
//    rt.spawn(async move {
//        ws.connect(Url::parse("wss://pubsub-edge.twitch.tv:443/v1/").unwrap())
//            .unwrap();
//        if let Err(e) = ws.run() {
//            println!("socket connect failed {}", e);
//        };
//    }).await;
//
//}

fn commands() -> HashMap<String, Box<dyn Command + Send + Sync>> {
    let mut commands: HashMap<String, Box<dyn Command + Send + Sync>> = HashMap::new();
    if let Ok(file) = std::fs::File::open("./oscommand.json") {
        let reader = std::io::BufReader::new(file);
        if let Ok(os_commands) = serde_json::from_reader::<std::io::BufReader<std::fs::File>, Vec<OSCommand>>(reader) {
            for os_command in os_commands.iter() {
                let title = &os_command.title;

                commands.insert(title.to_owned(), Box::new(os_command.clone()));
            };
        };
    };

    commands
}
