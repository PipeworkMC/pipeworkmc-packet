use crate::meta::*;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct C2SStatusPingPacket {
    pub timestamp : u64
}


impl Packet for C2SStatusPingPacket {
    const PREFIX : u8 = 1; // TODO: Use datagen.
    type Bound = Bound::C2S;
    type State = State::Status;
}
