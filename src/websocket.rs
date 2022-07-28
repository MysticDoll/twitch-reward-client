use crate::functions::Command;
use crate::twitch::AuthorizationRequest;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio::join;
use url::Url;
use serde_json::Value;
use std::collections::HashMap;

pub(crate) async fn connect_websocket(url: &str) -> std::result::Result<(tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::http::Response<()>), tokio_tungstenite::tungstenite::Error> {
    let connect_url = Url::parse(url).unwrap();
    connect_async(connect_url).await
}

pub(crate) async fn handle_websocket(auth_request: AuthorizationRequest, ws_stream: tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, commands: HashMap<String, Box<dyn Command + Send + Sync>>) {
    let (mut write, mut read) = ws_stream.split();

    if let Err(_e) = write.send(Message::Text(serde_json::to_string(&auth_request).expect("Error: Invalid AuthorizationRequest data."))).await {
        eprintln!("Error: websocket authorization is failed.");
    };

    let ping_handle = tokio::spawn(async move {
        loop {
            println!("ping");
            write.send(Message::Text("{\"type\": \"PING\"}".to_owned())).await;
            tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
        }
    });
    let reward_handle = tokio::spawn(async move {
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
                    if let Some(command) = commands.get(reward) {
                        match command.exec().await {
                            Ok(Err(e)) => {
                                if let Some(m) = e.stderr {
                                    eprintln!("{}", m)
                                }
                                // MEMO: this is useless when connecting to reward pub-sub.
                                //       this will be used with pub-sub of chats.
                                /***
                                if let Some(m) = e.message {
                                if let Err(_e) = write.send(Message::Text(format!("{}", m))).await {
                                eprintln!("Error: Failed to post message to chat");
                            };
                            };
                                 ***/
                            },
                            Ok(Ok(s)) => if let Some(m) = s.stdout {
                                println!("{}", m)
                            },
                            Err(_) => {
                                eprintln!("Error: Failed to spawn process for the command `{}`.", reward);
                            }
                        };
                    };
                }
            }

        };
    });

    join!(ping_handle, reward_handle);

}
