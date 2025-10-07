//! Serverbound configuration pong packet.


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
    pack_status::{
        PackStatus,
        PackStatusDecodeError
    },
    uuid::Uuid
};
use core::fmt::{ self, Display, Formatter };


/// Resource pack load status.
#[derive(Debug)]
pub struct C2SConfigResourcePackStatusPacket {
    /// UUID of the resource pack.
    pub uuid   : Uuid,
    /// Current status.
    pub status : PackStatus
}

impl PacketMeta for C2SConfigResourcePackStatusPacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("resource_pack");
}

impl PacketDecode for C2SConfigResourcePackStatusPacket {
    type Error = C2SConfigResourcePackStatusDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        uuid   : <_>::decode(iter).map_err(C2SConfigResourcePackStatusDecodeError::Uuid)?,
        status : <_>::decode(iter).map_err(C2SConfigResourcePackStatusDecodeError::Status)?
    }) }
}


/// Returned by packet decoders when a `C2SConfigResourcePackStatusPacket` was not decoded successfully.
#[derive(Debug)]
pub enum C2SConfigResourcePackStatusDecodeError {
    /// The pack UUID failed to decode.
    Uuid(IncompleteDecodeError),
    /// The pack status failed to decode.
    Status(PackStatusDecodeError)
}
impl Display for C2SConfigResourcePackStatusDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Uuid(err)   => write!(f, "uuid {err}"),
        Self::Status(err) => write!(f, "status {err}")
    } }
}
