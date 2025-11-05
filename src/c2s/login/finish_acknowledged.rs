use crate::meta::*;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct C2SLoginFinishAcknowledgedPacket;


impl Packet for C2SLoginFinishAcknowledgedPacket {
    const PREFIX : u8 = 0x03; // TODO: Use datagen.
    type Bound = Bound::C2S;
    type State = State::Login;
}
