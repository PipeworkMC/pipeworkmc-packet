use crate::meta::*;
use pipeworkmc_data::ident::Ident;
use std::borrow::Cow;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct C2SLoginCookieResponsePacket<'l> {
    pub key  : Ident,
    pub data : Option<Cow<'l, [u8]>>
}


impl Packet for C2SLoginCookieResponsePacket<'_> {
    const PREFIX : u8 = 0x04; // TODO: Use datagen.
    type Bound = Bound::C2S;
    type State = State::Login;
}
