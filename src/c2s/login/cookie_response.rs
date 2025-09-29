//! Serverbound login cookie response packet.


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
pub struct C2SLoginCookieResponsePacket {
    /// ID of the cookie.
    pub id   : Ident,
    /// Data of the cookie, if it exists.
    pub data : Option<Vec<u8>>
}

impl PacketMeta for C2SLoginCookieResponsePacket {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x04; // TODO: Check against current datagen.
}

impl PacketDecode for C2SLoginCookieResponsePacket {
    type Error = C2SLoginCookieResponseDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        id   : <_>::decode(iter).map_err(C2SLoginCookieResponseDecodeError::Id)?,
        data : <Option<Vec<u8>>>::decode(iter).map_err(C2SLoginCookieResponseDecodeError::Data)?
    }) }
}


/// Returned by packet decoders when a `C2SLoginCookieResponsePacket` was not decoded successfully.
#[derive(Debug)]
pub enum C2SLoginCookieResponseDecodeError {
    /// The cookie ID failed to decode.
    Id(IdentDecodeError),
    /// The cookie data failed to decode.
    Data(<Option<Vec<u8>> as PacketDecode>::Error)
}
impl Display for C2SLoginCookieResponseDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Id(err)   => write!(f, "id {err}"),
        Self::Data(err) => write!(f, "data {err}")
    } }
}
