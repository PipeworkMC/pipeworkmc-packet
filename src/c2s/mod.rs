use netzer::prelude::*;


pub mod handshake;
pub mod status;
pub mod login;
pub mod config;


#[derive(Clone, Debug, NetEncode)]
#[netzer(untagged)]
pub enum C2SPackets<'l> {
    Handshake(handshake::C2SHandshakePackets<'l>),
    Status(status::C2SStatusPackets),
    Login(login::C2SLoginPackets<'l>),
    Config(config::C2SConfigPackets<'l>)
}
