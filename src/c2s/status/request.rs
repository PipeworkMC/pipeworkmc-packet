use crate::meta::*;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct C2SStatusRequestPacket;


impl Packet for C2SStatusRequestPacket {
    const PREFIX : u8 = 0; // TODO: Use datagen.
    type Bound = Bound::C2S;
    type State = State::Status;
}
