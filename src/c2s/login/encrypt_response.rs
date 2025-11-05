use crate::meta::*;
use pipeworkmc_data::redacted::Redacted;
use std::borrow::Cow;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct C2SLoginEncryptResponsePacket<'l> {
    pub shared_secret : Redacted<Cow<'l, [u8]>>,
    pub verify_token  : Redacted<[u8; 4]>
}


impl Packet for C2SLoginEncryptResponsePacket<'_> {
    const PREFIX : u8 = 0x01; // TODO: Use datagen.
    type Bound = Bound::C2S;
    type State = State::Login;
}
