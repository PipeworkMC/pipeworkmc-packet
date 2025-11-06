use crate::meta::*;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct C2SConfigKeepAlivePacket {
    pub transaction : u64
}


impl Packet for C2SConfigKeepAlivePacket {
    const PREFIX : u8 = 0x04; // TODO: Use datagen.
    type Bound = Bound::C2S;
    type State = State::Config;
}
