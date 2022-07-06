use crate::client::TwitchRewardClient;
use crate::functions::Command;
use tokio::process::Command as TokioCommand;
use ws::Result;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct OSCommand {
    pub title: String,
    command: String,
    args: Vec<String>,
}

impl Command for OSCommand {
    fn exec(&self, _client: &TwitchRewardClient) -> Result<()> {
        let command= self.command.clone();
        let args = self.args.clone();

        let _t = tokio::spawn(async move {
            if let Err(e) = TokioCommand::new(&command)
               .args(args)
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
            {
                println!("failed exectuion {:?}", e);
                Err(e)
            } else {
                println!("success excution");
                Ok(())
            };
        });

        Ok(())
    }
}

impl OSCommand{
    pub fn new(title: &str, command: &str, args: Vec<String>) -> Self {
        Self {
            title: title.into(),
            command: command.into(),
            args
        }
    }
}
