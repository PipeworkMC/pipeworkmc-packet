use crate::meta::*;
use pipeworkmc_data::profile::AccountProfile;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct S2CLoginFinishedPacket {
    pub profile : AccountProfile
}


impl Packet for S2CLoginFinishedPacket {
    const PREFIX : u8 = 0x02; // TODO: Use datagen.
    type Bound = Bound::S2C;
    type State = State::Login;
}
