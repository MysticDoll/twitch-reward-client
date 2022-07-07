pub(crate) mod http_request;
pub(crate) mod oscommand;

use crate::client::TwitchRewardClient;
use ws::Result as WSResult;

pub trait Command: std::fmt::Debug {
    fn exec(&self, client: &TwitchRewardClient) -> WSResult<()>;
}
