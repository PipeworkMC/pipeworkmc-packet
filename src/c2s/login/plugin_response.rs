use crate::meta::*;
use pipeworkmc_data::{
    Minecraft,
    unprefixed::{
        AllSlice,
        AllVec
    }
};
use std::borrow::Cow;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct C2SLoginPluginResponsePacket<'l> {
    #[netzer(convert = "VarInt<u32>")]
    pub transaction : u32,
    #[netzer(encode_with = "encode_data", decode_with = "decode_data")]
    pub data        : Option<Cow<'l, [u8]>>
}

async fn encode_data<W : netzer::AsyncWrite>(v : &Option<Cow<'_, [u8]>>, w : W) -> netzer::Result {
    let data = v.as_ref().map(|v| <&AllSlice<u8>>::from(&**v));
    <Option<&AllSlice<u8>> as NetEncode<Minecraft>>::encode(&data, w).await
}
async fn decode_data<'l, R : netzer::AsyncRead>(r : R) -> netzer::Result<Option<Cow<'l, [u8]>>> {
    let v = <Option<AllVec<u8>> as NetDecode<Minecraft>>::decode(r).await?;
    Ok(v.map(|data| Cow::Owned(data.0)))
}


impl Packet for C2SLoginPluginResponsePacket<'_> {
    const PREFIX : u8 = 0x02; // TODO: Use datagen.
    type Bound = Bound::C2S;
    type State = State::Login;
}
