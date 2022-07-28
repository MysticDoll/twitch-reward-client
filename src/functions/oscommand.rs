use crate::functions::{Command, CommandSuccess, CommandErr};
use tokio::process::Command as TokioCommand;
use tokio::task::JoinHandle;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct OSCommand {
    pub title: String,
    command: String,
    args: Vec<String>,
}

impl Command for OSCommand {
    fn exec(&self) -> JoinHandle<Result<CommandSuccess, CommandErr>> {
        let title = self.title.clone();
        let command= self.command.clone();
        let args = self.args.clone();
        println!("{:?}", &self);

        tokio::spawn(async move {
            if let Err(_e) = TokioCommand::new(&command)
               .args(&args)
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
            {
                Err(CommandErr::err(&format!(
                    "Error: Failed to execute command {} with args {:?}",
                    title,
                    args
                )))
            } else {
                Ok(CommandSuccess::nil())
            }
        })
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
