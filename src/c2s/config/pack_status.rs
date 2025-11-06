use crate::meta::*;
use pipeworkmc_data::{
    pack_status::PackStatus,
    uuid::Uuid
};
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct C2SConfigPackStatusPacket {
    pub pack   : Uuid,
    pub status : PackStatus
}


impl Packet for C2SConfigPackStatusPacket {
    const PREFIX : u8 = 0x06; // TODO: Use datagen.
    type Bound = Bound::C2S;
    type State = State::Config;
}
