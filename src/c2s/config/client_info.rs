use crate::meta::*;
use pipeworkmc_data::client_info::ClientInfo;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct C2SConfigClientInfoPacket {
    pub info : ClientInfo
}


impl Packet for C2SConfigClientInfoPacket {
    const PREFIX : u8 = 0x00; // TODO: Use datagen.
    type Bound = Bound::C2S;
    type State = State::Config;
}
