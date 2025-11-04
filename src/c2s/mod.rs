use netzer::prelude::*;


pub mod handshake;
pub mod status;


#[derive(Clone, Debug, NetEncode)]
#[netzer(untagged)]
pub enum C2SPackets<'l> {
    Handshake(handshake::C2SHandshakePackets<'l>),
    Status(status::C2SStatusPackets)
}
