use crate::ics03_connection::msgs::ConnectionMsg;
use crate::ics04_channel::msgs::ChannelMsg;
use crate::{
    application::ics20_fungible_token_transfer::msgs::transfer::MsgTransfer,
    ics02_client::msgs::ClientMsg,
};

/// Enumeration of all messages that the local ICS26 module is capable of routing.
#[derive(Clone, Debug)]
pub enum Ics26Envelope {
    Ics2Msg(ClientMsg),
    Ics3Msg(ConnectionMsg),
    Ics4Msg(ChannelMsg),
    Ics20Msg(MsgTransfer),
}
