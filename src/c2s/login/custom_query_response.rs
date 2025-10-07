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
    /// Transaction ID of the request.
    pub transaction : u32,
    /// Response data, if it exists.
    pub data        : Option<Vec<u8>>
}

impl PacketMeta for C2SLoginCustomQueryResponsePacket {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("custom_query_answer");
}

impl PacketDecode for C2SLoginCustomQueryResponsePacket {
    type Error = C2SLoginCustomQueryResponseDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        transaction : <VarInt<u32>>::decode(iter).map_err(C2SLoginCustomQueryResponseDecodeError::Transaction)?.0,
        data        : bool::decode(iter).map_err(C2SLoginCustomQueryResponseDecodeError::DataVariant)?
            .then(|| iter.collect::<Vec<_>>())
    }) }
}


/// Returned by packet decoders when a `C2SLoginCookieResponsePacket` was not decoded successfully.
#[derive(Debug)]
pub enum C2SLoginCustomQueryResponseDecodeError {
    /// The request transaction ID failed to decode.
    Transaction(VarIntDecodeError),
    /// The response data option variant failed to decode.
    DataVariant(IncompleteDecodeError)
}
impl Display for C2SLoginCustomQueryResponseDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Transaction(err) => write!(f, "transaction {err}"),
        Self::DataVariant(err) => write!(f, "data {err}")
    } }
}
