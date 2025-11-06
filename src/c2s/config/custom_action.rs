use crate::meta::*;
use pipeworkmc_data::{
    ident::Ident,
    nbt::NbtElement
};
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct C2SConfigCustomActionPacket<'l> {
    pub id   : Ident,
    pub data : NbtElement<'l>
}


impl Packet for C2SConfigCustomActionPacket<'_> {
    const PREFIX : u8 = 0x08; // TODO: Use datagen.
    type Bound = Bound::C2S;
    type State = State::Config;
}
