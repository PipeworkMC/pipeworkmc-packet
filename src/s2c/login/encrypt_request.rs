use crate::meta::*;
use pipeworkmc_data::{
    bounded_string::BoundedString,
    redacted::Redacted
};
use std::borrow::Cow;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct S2CLoginEncryptRequestPacket<'l> {
    pub server_id       : BoundedString<20>,
    pub public_key      : Redacted<Cow<'l, [u8]>>,
    pub verify_token    : Redacted<[u8; 4]>,
    pub mojauth_enabled : bool
}


impl Packet for S2CLoginEncryptRequestPacket<'_> {
    const PREFIX : u8 = 0x01; // TODO: Use datagen.
    type Bound = Bound::S2C;
    type State = State::Login;
}
