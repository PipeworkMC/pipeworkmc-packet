//! Serverbound login custom query response packet.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter,
        IncompleteDecodeError
    },
    meta::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};
use pipeworkmc_data::varint::{
    VarInt,
    VarIntDecodeError
};
use core::fmt::{ self, Display, Formatter };


/// The server previously made a custom query.
#[derive(Debug)]
pub struct C2SLoginCustomQueryResponsePacket {
    /// ID of the request.
    pub id   : u32,
    /// Response data, if it exists.
    pub data : Option<Vec<u8>>
}

impl PacketMeta for C2SLoginCustomQueryResponsePacket {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x02; // TODO: Check against current datagen.
}

impl PacketDecode for C2SLoginCustomQueryResponsePacket {
    type Error = C2SLoginCustomQueryResponseDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        id   : <VarInt<u32>>::decode(iter).map_err(C2SLoginCustomQueryResponseDecodeError::Id)?.0,
        data : bool::decode(iter).map_err(C2SLoginCustomQueryResponseDecodeError::DataVariant)?
            .then(|| iter.collect::<Vec<_>>())
    }) }
}


/// Returned by packet decoders when a `C2SLoginCookieResponsePacket` was not decoded successfully.
#[derive(Debug)]
pub enum C2SLoginCustomQueryResponseDecodeError {
    /// The request ID failed to decode.
    Id(VarIntDecodeError),
    /// The response data option variant failed to decode.
    DataVariant(IncompleteDecodeError)
}
impl Display for C2SLoginCustomQueryResponseDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Id(err)   => write!(f, "id {err}"),
        Self::DataVariant(err) => write!(f, "data {err}")
    } }
}
