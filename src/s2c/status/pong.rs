use crate::meta::*;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct S2CStatusPongPacket {
    pub timestamp : u64
}


impl Packet for S2CStatusPongPacket {
    const PREFIX : u8 = 0x01; // TODO: Use datagen.
    type Bound = Bound::S2C;
    type State = State::Status;
}
