//! Serverbound login start packet.


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
use pipeworkmc_data::{
    bounded_string::{
        BoundedString,
        BoundedStringDecodeError
    },
    uuid::Uuid
};
use core::fmt::{ self, Display, Formatter };


/// Informs the server that the client would like to log in.
#[derive(Debug)]
pub struct C2SLoginStartPacket {
    /// The username that the client would like to log in with.
    pub username : BoundedString<16>,
    /// The account UUID that the client would like to log in with.
    pub uuid     : Uuid
}

impl PacketMeta for C2SLoginStartPacket {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x00; // TODO: Check against current datagen.
}

impl PacketDecode for C2SLoginStartPacket {
    type Error = C2SLoginStartDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        username : <_>::decode(iter).map_err(C2SLoginStartDecodeError::Username)?, // TODO: Validate username characters
        uuid     : <_>::decode(iter).map_err(C2SLoginStartDecodeError::Uuid)?
    }) }
}


/// Returned by packet decoders when a `C2SLoginStartPacket` was not decoded successfully.
#[derive(Debug)]
pub enum C2SLoginStartDecodeError {
    /// The username failed to decode.
    Username(BoundedStringDecodeError),
    /// The UUID failed to decode.
    Uuid(IncompleteDecodeError)
}
impl Display for C2SLoginStartDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Username(err) => write!(f, "username {err}"),
        Self::Uuid(err)     => write!(f, "uuid {err}")
    } }
}
