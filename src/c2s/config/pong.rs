use crate::meta::*;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct C2SConfigPongPacket {
    pub transaction : u32
}


impl Packet for C2SConfigPongPacket {
    const PREFIX : u8 = 0x05; // TODO: Use datagen.
    type Bound = Bound::C2S;
    type State = State::Config;
}
