use crate::meta::*;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct S2CLoginCompressionPacket {
    #[netzer(convert = "VarInt<u32>")]
    pub threshold : u32
}


impl Packet for S2CLoginCompressionPacket {
    const PREFIX : u8 = 0x03; // TODO: Use datagen.
    type Bound = Bound::S2C;
    type State = State::Login;
}
