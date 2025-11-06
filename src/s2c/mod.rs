use netzer::prelude::*;


pub mod status;
pub mod login;
pub mod config;


#[derive(Clone, Debug, NetEncode)]
#[netzer(untagged)]
pub enum S2CPackets<'l> {
    Status(status::S2CStatusPackets<'l>),
    Login(login::S2CLoginPackets<'l>),
    Config(config::S2CConfigPackets)
}
