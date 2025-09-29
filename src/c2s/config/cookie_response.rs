//! Serverbound configuration cookie response packet.


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
use pipeworkmc_data::ident::{
    Ident,
    IdentDecodeError
};
use core::fmt::{ self, Display, Formatter };


/// The server previously requested the value of a cookie.
#[derive(Debug)]
pub struct C2SConfigCookieResponsePacket {
    /// ID of the cookie.
    pub id   : Ident,
    /// Data of the cookie, if it exists.
    pub data : Option<Vec<u8>>
}

impl PacketMeta for C2SConfigCookieResponsePacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x01; // TODO: Check against current datagen.
}

impl PacketDecode for C2SConfigCookieResponsePacket {
    type Error = C2SConfigCookieResponseDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        id   : <_>::decode(iter).map_err(C2SConfigCookieResponseDecodeError::Id)?,
        data : <Option<Vec<u8>>>::decode(iter).map_err(C2SConfigCookieResponseDecodeError::Data)?
    }) }
}


/// Returned by packet decoders when a `C2SConfigCookieResponsePacket` was not decoded successfully.
#[derive(Debug)]
pub enum C2SConfigCookieResponseDecodeError {
    /// The cookie id failed to decode.
    Id(IdentDecodeError),
    /// The cookie data failed to decode.
    Data(<Option<Vec<u8>> as PacketDecode>::Error)
}
impl Display for C2SConfigCookieResponseDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Id(err)   => write!(f, "id {err}"),
        Self::Data(err) => write!(f, "data {err}")
    } }
}
