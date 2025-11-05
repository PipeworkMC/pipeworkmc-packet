use crate::meta::*;
use pipeworkmc_data::text::Text;
use std::borrow::Cow;
use netzer::prelude::*;
use serde_json::to_string as json_to_string;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct S2CLoginDisconnectPacket<'l> {
    pub message_json : Cow<'l, str>
}


impl Packet for S2CLoginDisconnectPacket<'_> {
    const PREFIX : u8 = 0x00; // TODO: Use datagen.
    type Bound = Bound::S2C;
    type State = State::Login;
}


impl From<&Text> for S2CLoginDisconnectPacket<'_> {
    fn from(value : &Text) -> Self {
        Self { message_json : Cow::Owned(json_to_string(value).unwrap()) }
    }
}
