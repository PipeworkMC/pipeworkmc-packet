use crate::meta::*;
use pipeworkmc_data::{
    Minecraft,
    ident::Ident,
    unprefixed::{
        AllSlice,
        AllVec
    }
};
use std::borrow::Cow;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct S2CLoginPluginRequestPacket<'l> {
    #[netzer(convert = "VarInt<u32>")]
    pub transaction : u32,
    pub channel     : Ident,
    #[netzer(encode_with = "encode_data", decode_with = "decode_data")]
    pub data        : Cow<'l, [u8]>
}

async fn encode_data<W : netzer::AsyncWrite>(v : &Cow<'_, [u8]>, w : W) -> netzer::Result {
    <AllSlice<u8> as NetEncode<Minecraft>>::encode(<&AllSlice<u8>>::from(&**v), w).await
}
async fn decode_data<'l, R : netzer::AsyncRead>(r : R) -> netzer::Result<Cow<'l, [u8]>> {
    Ok(Cow::Owned(<AllVec<u8> as NetDecode<Minecraft>>::decode(r).await?.0))
}


impl Packet for S2CLoginPluginRequestPacket<'_> {
    const PREFIX : u8 = 0x04; // TODO: Use datagen.
    type Bound = Bound::S2C;
    type State = State::Login;
}
