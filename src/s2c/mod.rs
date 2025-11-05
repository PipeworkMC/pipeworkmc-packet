use netzer::prelude::*;


pub mod status;
pub mod login;


#[derive(Clone, Debug, NetEncode)]
#[netzer(untagged)]
pub enum S2CPackets<'l> {
    Status(status::S2CStatusPackets<'l>),
    Login(login::S2CLoginPackets<'l>)
}
