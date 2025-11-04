use netzer::prelude::*;


pub mod status;


#[derive(Clone, Debug, NetEncode)]
#[netzer(untagged)]
pub enum S2CPackets<'l> {
    Status(status::S2CStatusPackets<'l>)
}
