use crate::meta::*;
use pipeworkmc_data::{
    bounded_string::BoundedString,
    uuid::Uuid
};
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct C2SLoginStartPacket {
    pub username : BoundedString<16>, // TODO: Validate username characters.
    pub uuid     : Uuid
}


impl Packet for C2SLoginStartPacket {
    const PREFIX : u8 = 0x00; // TODO: Use datagen.
    type Bound = Bound::C2S;
    type State = State::Login;
}
