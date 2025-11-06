use crate::meta::*;
use pipeworkmc_data::known_pack::KnownPack;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct C2SConfigKnownPacksPacket<'l> {
    pub packs : Vec<KnownPack<'l>>
}


impl Packet for C2SConfigKnownPacksPacket<'_> {
    const PREFIX : u8 = 0x07; // TODO: Use datagen.
    type Bound = Bound::C2S;
    type State = State::Config;
}
