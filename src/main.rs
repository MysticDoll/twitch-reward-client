#[macro_use]
extern crate lazy_static;
extern crate env_logger;

mod client;
mod functions;
mod twitch;

use crate::client::TwitchRewardClient;
use crate::functions::Command;
use crate::functions::oscommand::OSCommand;
use std::collections::HashMap;
use url::Url;
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() {
    env_logger::init();
    connect().await;
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
