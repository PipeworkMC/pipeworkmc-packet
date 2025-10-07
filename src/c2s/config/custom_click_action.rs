//! Serverbound configuration custom click action packet.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter
    },
    meta::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};
use pipeworkmc_data::{
    ident::Ident,
    nbt::NbtElement
};


/// Sent when the client triggers a `minecraft:custom` click action.
#[derive(Debug)]
pub struct C2SConfigCustomClickActionPacket {
    /// Id of the click action.
    pub id      : Ident,
    pub payload : NbtElement<'static>
}

impl PacketMeta for C2SConfigCustomClickActionPacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("client_information");
}

impl PacketDecode for C2SConfigCustomClickActionPacket {
    type Error = ClientInfoDecodeError;

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        info : <_>::decode(iter)?
    }) }
}
