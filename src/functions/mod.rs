pub(crate) mod http_request;
pub(crate) mod oscommand;

use tokio::task::JoinHandle;

pub trait Command: std::fmt::Debug {
    fn exec(&self) -> JoinHandle<Result<CommandSuccess, CommandErr>>;
}

pub struct CommandSuccess {
    pub message: Option<String>,
    pub stdout: Option<String>,
}

impl CommandSuccess {
    pub fn new(message: &str, stdout: &str) -> Self {
        Self {
            message: Some(message.to_owned()),
            stdout: Some(stdout.to_owned()),
        }
    }

    pub fn message(message: &str) -> Self {
        Self {
            message: Some(message.to_owned()),
            stdout: None,
        }
    }

    pub fn info(info: &str) -> Self {
        Self {
            message: None,
            stdout: Some(info.to_owned()),
        }
    }

    pub fn nil() -> Self {
        Self {
            message: None,
            stdout: None,
        }
    }
}

pub struct CommandErr {
    pub message: Option<String>,
    pub stderr: Option<String>,
}

impl CommandErr {
    pub fn new(message: String, stderr: String) -> Self {
        Self {
            message: Some(message),
            stderr: Some(stderr),
        }
    }

    pub fn message(message: &str) -> Self {
        Self {
            message: Some(message.to_owned()),
            stderr: None,
        }
    }

    pub fn err(err: &str) -> Self {
        Self {
            message: None,
            stderr: Some(err.to_owned()),
        }
    }

    pub fn nil() -> Self {
        Self {
            message: None,
            stderr: None,
        }
    }
}
