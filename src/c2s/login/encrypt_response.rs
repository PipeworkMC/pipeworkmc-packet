//! Serverbound login encrypt response packet.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter,
        vec::VecDecodeError
    },
    meta::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};
use pipeworkmc_data::redacted::Redacted;
use core::fmt::{ self, Display, Formatter };


/// Informs the server of the secret key to use for all future communication.
#[derive(Debug)]
pub struct C2SLoginEncryptResponsePacket {
    /// The secret key, encrypted using the server's public key.
    pub encrypted_secret_key : Redacted<Vec<u8>>,
    /// The previously sent verify token, now encrypted using the server's public key.
    pub encrypted_vtoken     : Vec<u8>
}

impl PacketMeta for C2SLoginEncryptResponsePacket {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x01; // TODO: Check against current datagen.
}

impl PacketDecode for C2SLoginEncryptResponsePacket {
    type Error = C2SLoginEncryptResponseDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        encrypted_secret_key : Redacted::from(<Vec<u8>>::decode(iter).map_err(C2SLoginEncryptResponseDecodeError::SecretKey)?),
        encrypted_vtoken     : <_>::decode(iter).map_err(C2SLoginEncryptResponseDecodeError::VerifyToken)?
    }) }
}


/// Returned by packet decoders when a `C2SLoginEncryptResponsePacket` was not decoded successfully.
#[derive(Debug)]
pub enum C2SLoginEncryptResponseDecodeError {
    /// The secret key failed to decode.
    SecretKey(VecDecodeError<<u8 as PacketDecode>::Error>),
    /// The verify token failed to decode.
    VerifyToken(VecDecodeError<<u8 as PacketDecode>::Error>)
}
impl Display for C2SLoginEncryptResponseDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::SecretKey(err)   => write!(f, "secret key {err}"),
        Self::VerifyToken(err) => write!(f, "verify token {err}")
    } }
}
