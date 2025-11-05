use crate::meta::*;
use pipeworkmc_data::ident::Ident;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct C2SLoginCookieRequestPacket {
    pub key : Ident
}


impl Packet for C2SLoginCookieRequestPacket {
    const PREFIX : u8 = 0x05; // TODO: Use datagen.
    type Bound = Bound::S2C;
    type State = State::Login;
}
