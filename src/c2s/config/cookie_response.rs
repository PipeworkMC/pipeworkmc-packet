use crate::meta::*;
use pipeworkmc_data::ident::Ident;
use std::borrow::Cow;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct C2SConfigCookieResponsePacket<'l> {
    pub key  : Ident,
    pub data : Option<Cow<'l, [u8]>>
}


impl Packet for C2SConfigCookieResponsePacket<'_> {
    const PREFIX : u8 = 0x01; // TODO: Use datagen.
    type Bound = Bound::C2S;
    type State = State::Config;
}
