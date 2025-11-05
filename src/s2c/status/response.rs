use crate::meta::*;
use pipeworkmc_data::server_status::Status;
use std::borrow::Cow;
use netzer::prelude::*;
use serde_json::to_string as json_to_string;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct S2CStatusResponsePacket<'l> {
    pub status_json : Cow<'l, str>
}


impl Packet for S2CStatusResponsePacket<'_> {
    const PREFIX : u8 = 0x00; // TODO: Use datagen.
    type Bound = Bound::S2C;
    type State = State::Status;
}


impl From<&Status<'_>> for S2CStatusResponsePacket<'_> {
    fn from(value : &Status) -> Self {
        Self { status_json : Cow::Owned(json_to_string(value).unwrap()) }
    }
}
